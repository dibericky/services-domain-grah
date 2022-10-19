use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Service {
    id: String,
}

impl Service {
    fn new(name: String) -> Self {
        Self { id: name }
    }

    fn name(&self) -> &str {
        &self.id
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Domain(String);
impl Domain {
    fn key(&self) -> &str {
        &self.0
    }
}

#[derive(Default)]
pub struct Repository<'a, 'b> {
    services: HashMap<&'a str, &'a Service>,
    domains: HashSet<&'b str>,
    service_domains: Vec<(&'a str, &'b Domain)>
}

impl <'a, 'b> Repository<'a, 'b> {
    pub fn add_service (&mut self, service: &'a Service) {
        self.services.insert(&service.id, service);
    }

    pub fn add_domain(&mut self, service: &'a str, domain: &'b Domain) {
        self.domains.insert(domain.key());
        self.service_domains.push((service, domain));
    }

    pub fn get_service_domains(&self, service: &'a str) -> Vec<&Domain> {
        self.service_domains
            .iter()
            .filter_map(|el|if el.0 == service { Some(el.1) } else { None })
            .collect()
    }

    pub fn get_services_with_domain(&self, domain: &Domain) -> Vec<&Service> {
        self.service_domains
            .iter()
            .filter_map(|el|{
                if el.1 != domain {
                    return None;
                }

                let service : &Service = self.services.get(el.0).expect(&format!("{} service does not exist", el.0));
                Some(service)
            })
            .collect()
    }

    #[cfg(test)]
    pub fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }
}

#[cfg(test)]
mod test {
    use super::{Repository, Service, Domain};


    #[test]
    fn add_services() {
        let mut repo = Repository::default();
        let service = Service::new("service-1".to_string());
        let has_service = repo.has_service(service.name());
        assert!(!has_service);
        repo.add_service(&service);
        let has_service = repo.has_service(service.name());
        assert!(has_service);
    }

    #[test]
    fn add_and_get_domains() {
        let mut repo = Repository::default();
        let service_1 = Service::new("service-1".to_string());
        let service_2 = Service::new("service-2".to_string());
        repo.add_service(&service_1);
        repo.add_service(&service_2);
        let domain_a = Domain("a".to_string());
        let domain_b = Domain("b".to_string());
        let domain_c = Domain("c".to_string());
        repo.add_domain(service_1.name(), &domain_a);
        repo.add_domain(service_1.name(), &domain_b);
        repo.add_domain(service_2.name(), &domain_b);
        repo.add_domain(service_2.name(), &domain_c);

        assert_eq!(repo.get_service_domains(service_1.name()), vec![&domain_a, &domain_b]);
    }

    #[test]
    fn get_service_with_domains() {
        let mut repo = Repository::default();
        let service_1 = Service::new("service-1".to_string());
        let service_2 = Service::new("service-2".to_string());
        repo.add_service(&service_1);
        repo.add_service(&service_2);
        let domain_a = Domain("a".to_string());
        let domain_b = Domain("b".to_string());
        let domain_c = Domain("c".to_string());
        repo.add_domain(service_1.name(), &domain_a);
        repo.add_domain(service_1.name(), &domain_b);
        repo.add_domain(service_2.name(), &domain_b);
        repo.add_domain(service_2.name(), &domain_c);

        assert_eq!(repo.get_services_with_domain(&domain_b), vec![&service_1, &service_2]);
        assert_eq!(repo.get_services_with_domain(&domain_a), vec![&service_1]);
        assert_eq!(repo.get_services_with_domain(&domain_c), vec![&service_2]);
    }
}