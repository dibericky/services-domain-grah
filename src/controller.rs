use anyhow::Result;

use crate::repository::{Domain, Repository, Service as RepoService};

struct Controller<'a> {
    repository: Repository<'a>,
}
struct Service {
    name: String,
    domains: Vec<String>,
}

impl From<&String> for Domain {
    fn from(s: &String) -> Self {
        Self(s.to_owned())
    }
}

impl From<&Service> for RepoService {
    fn from(s: &Service) -> Self {
        Self::new(s.name.clone())
    }
}

impl<'a> Controller<'a> {
    fn new(repository: Repository<'a>) -> Self {
        Self { repository }
    }

    fn add_service(&mut self, service: &'a Service) -> Result<()> {
        self.repository.add_service(service.into())?;
        for domain in &service.domains {
            self.repository.add_domain(&service.name, domain.into());
        }

        Ok(())
    }

    fn link_services(&mut self, from: &'a Service, to: &'a Service) {
        self.repository.add_link(&from.name, &to.name);
    }

    fn get_connected_domains(&self, service: &Service) -> Vec<String> {
        let domains = self
            .repository
            .get_service_domains(&service.name)
            .iter()
            .map(|domain| domain.key().to_owned())
            .collect();

        let connected_domains = self
            .repository
            .get_links(&service.name)
            .iter()
            .map(|&service| {
                let service_domains: Vec<&Domain> =
                    self.repository.get_service_domains(service.name());
                service_domains
                    .iter()
                    .map(|dom| dom.key().to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
            .concat();

        [domains, connected_domains].concat()
    }
}

impl Service {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            domains: vec![],
        }
    }

    fn add_domain(&mut self, domain: &str) {
        self.domains.push(domain.to_owned())
    }

    fn get_domains(&self) -> &Vec<String> {
        &self.domains
    }
}

#[cfg(test)]
mod test {
    use crate::repository::Repository;

    use super::{Controller, Service};

    #[test]
    fn get_domains_connected_to_service() {
        let mut ctrl = Controller::new(Repository::default());
        let mut service1 = Service::new("s1");
        service1.add_domain("dom1");
        service1.add_domain("dom2");
        ctrl.add_service(&service1)
            .expect("failed adding service 1");

        let mut service2 = Service::new("s2");
        service2.add_domain("dom3");
        service2.add_domain("dom4");
        ctrl.add_service(&service2)
            .expect("failed adding service 2");

        let mut service3 = Service::new("s3");
        service3.add_domain("dom5");
        ctrl.add_service(&service3)
            .expect("failed adding service 3");

        ctrl.link_services(&service1, &service2);

        assert_eq!(
            ctrl.get_connected_domains(&service1),
            vec!["dom1", "dom2", "dom3", "dom4"]
        )
    }
}
