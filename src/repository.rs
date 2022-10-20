use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Service {
    id: String,
}

impl Service {
    pub fn new(name: String) -> Self {
        Self { id: name }
    }

    pub fn name(&self) -> &str {
        &self.id
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Domain(pub String);
impl Domain {
    pub fn key(&self) -> &str {
        &self.0
    }
}

#[derive(Default, Debug)]
pub struct Repository<'a> {
    services: HashMap<String, Service>,
    domains: HashSet<String>,
    service_domains: Vec<(&'a str, Domain)>,
    links: Vec<(&'a str, &'a str)>,
}

impl<'a> Repository<'a> {
    pub fn add_service(&mut self, service: Service) -> Result<()> {
        if self.has_service(service.name()) {
            return Err(anyhow!("{} service already exists", service.name()));
        }
        self.services.insert(service.id.clone(), service);

        Ok(())
    }

    pub fn add_domain(&mut self, service: &'a str, domain: Domain) {
        self.domains.insert(domain.key().to_owned());
        self.service_domains.push((service, domain));
    }

    pub fn get_service_domains(&self, service: &'a str) -> Vec<&Domain> {
        self.service_domains
            .iter()
            .filter_map(|el| if el.0 == service { Some(&el.1) } else { None })
            .collect()
    }
    pub fn add_link(&mut self, from: &'a str, to: &'a str) {
        self.links.push((from, to))
    }

    pub fn get_links(&self, service: &str) -> Vec<&Service> {
        println!("{:?}", self);
        let services: Vec<&Service> = self
            .links
            .iter()
            .filter_map(|&link| match link {
                (from, to) if from == service => Some(to),
                (from, to) if to == service => Some(from),
                _ => None,
            })
            .map(|s| {
                self.services
                    .get(s)
                    .unwrap_or_else(|| panic!("{s} expected to exist"))
            })
            .collect();

        services
    }

    pub fn get_services_with_domain(&self, domain: &Domain) -> Vec<&Service> {
        self.service_domains
            .iter()
            .filter_map(|el| {
                if &el.1 != domain {
                    return None;
                }

                let service: &Service = self
                    .services
                    .get(el.0)
                    .unwrap_or_else(|| panic!("{} expected to exist", el.0));
                Some(service)
            })
            .collect()
    }

    pub fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }
}

#[cfg(test)]
mod test {
    use super::{Domain, Repository, Service};

    #[test]
    fn add_services() {
        let mut repo = Repository::default();
        let service = Service::new("service-1".to_string());
        let name = service.name().to_owned();
        let has_service = repo.has_service(service.name());
        assert!(!has_service);
        repo.add_service(service).unwrap();
        let has_service = repo.has_service(&name);
        assert!(has_service);
    }

    #[test]
    fn add_services_err_if_already_exists() {
        let mut repo = Repository::default();

        let service = Service::new("service-1".to_string());
        repo.add_service(service).unwrap();

        let service = Service::new("service-1".to_string());
        assert!(repo.add_service(service).is_err());
    }

    #[test]
    fn add_and_get_domains() {
        let mut repo = Repository::default();

        let service_1 = Service::new("service-1".to_string());
        let service_2 = Service::new("service-2".to_string());
        let name_1 = service_1.name().to_owned();
        let name_2 = service_2.name().to_owned();

        repo.add_service(service_1).unwrap();
        repo.add_service(service_2).unwrap();

        let domain_a = Domain("a".to_string());
        let domain_b = Domain("b".to_string());
        let domain_c = Domain("c".to_string());

        repo.add_domain(&name_1, domain_a.clone());
        repo.add_domain(&name_1, domain_b.clone());

        repo.add_domain(&name_2, domain_b.clone());
        repo.add_domain(&name_2, domain_c);

        assert_eq!(
            repo.get_service_domains(&name_1),
            vec![&domain_a, &domain_b]
        );
    }

    #[test]
    fn get_service_with_domains() {
        let mut repo = Repository::default();
        let service_1 = Service::new("service-1".to_string());
        let service_2 = Service::new("service-2".to_string());
        let name_1 = service_1.name().to_owned();
        let name_2 = service_2.name().to_owned();

        repo.add_service(service_1.clone()).unwrap();
        repo.add_service(service_2.clone()).unwrap();

        let domain_a = Domain("a".to_string());
        let domain_b = Domain("b".to_string());
        let domain_c = Domain("c".to_string());
        repo.add_domain(&name_1, domain_a.clone());
        repo.add_domain(&name_1, domain_b.clone());
        repo.add_domain(&name_2, domain_b.clone());
        repo.add_domain(&name_2, domain_c.clone());

        assert_eq!(
            repo.get_services_with_domain(&domain_b),
            vec![&service_1, &service_2]
        );
        assert_eq!(repo.get_services_with_domain(&domain_a), vec![&service_1]);
        assert_eq!(repo.get_services_with_domain(&domain_c), vec![&service_2]);
    }

    #[test]
    fn add_link_services() {
        let mut repo = Repository::default();
        let service_1 = Service::new("service-1".to_string());
        let service_2 = Service::new("service-2".to_string());
        let service_3 = Service::new("service-3".to_string());

        let name_1 = service_1.name().to_owned();
        let name_2 = service_2.name().to_owned();
        let name_3 = service_3.name().to_owned();

        let copy_service_1 = service_1.clone();
        let copy_service_2 = service_2.clone();
        let copy_service_3 = service_3.clone();

        repo.add_service(service_1).unwrap();
        repo.add_service(service_2).unwrap();
        repo.add_service(service_3).unwrap();

        repo.add_link(&name_1, &name_3);
        repo.add_link(&name_2, &name_3);

        assert_eq!(repo.get_links(&name_1), vec![&copy_service_3]);
        assert_eq!(
            repo.get_links(&name_3),
            vec![&copy_service_1, &copy_service_2]
        );
    }
}
