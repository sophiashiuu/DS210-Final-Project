use std::collections::{HashMap, HashSet};

// Node structure representing a company
#[derive(Debug)]
struct CompanyNode {
    id: String, 
    locations: HashSet<String>, // Set of locations where layoffs occurred for the company
}

impl CompanyNode {
    fn new(id: String) -> Self {
        CompanyNode {
            id,
            locations: HashSet::new(),
        }
    }
}

// Graph structure representing layoffs since COVID-19
#[derive(Debug)]
struct LayoffGraph {
    companies: HashMap<String, CompanyNode>, 
}

impl LayoffGraph {
    fn new() -> Self {
        LayoffGraph {
            companies: HashMap::new(),
        }
    }

    // Add company node to the graph
    fn add_company(&mut self, id: String) {
        if !self.companies.contains_key(&id) {
            self.companies.insert(id.clone(), CompanyNode::new(id));
        }
    }

    // Add location edge between a company and a location
    fn add_location_edge(&mut self, company_id: String, location: String) {
        if let Some(company) = self.companies.get_mut(&company_id) {
            company.locations.insert(location);
        }
    }

    // Display the graph
    fn display(&self) {
        for (company_id, company) in &self.companies {
            println!("Company: {}", company_id);
            for location in &company.locations {
                println!("  -> Location: {}", location);
            }
        }
    }
}

