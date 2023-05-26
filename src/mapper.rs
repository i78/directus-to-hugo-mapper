use std::collections::HashMap;
use std::fs;
use serde_json::{Value};
use slugify::slugify;

trait CanProvideSlugForItself {
    fn slug(&self) -> String;
}

impl CanProvideSlugForItself for Value {
    fn slug(&self) -> String {
        slugify!(self["name"].as_str().expect("nono"))
    }
}

fn filename(slug: &str, prefix: &String, suffix: &str) -> String {
    format!("{}{}{}", prefix, slug, suffix)
}

fn map_to_frontmatter(mapping_config: &HashMap<String, String>, value: &Value) -> Result<String, ()> {
    let is_header_field = |name: &String| -> bool { name.as_str() != "body" };

    let to_frontmatter_header = |k: &String, v: &String| -> String { format!("{}={}\n", k, value[v]) };

    let body = &mapping_config.get("body")
        .and_then(|k| value[k].as_str());

    let meta = mapping_config.iter()
        .filter(|(k, _)| is_header_field(&k))
        .map(|(k, v)| to_frontmatter_header(k, v))
        .reduce(|cur: String, nxt: String| cur + &nxt);

    match (meta, body) {
        (Some(meta), Some(body)) => Ok(format!("+++\n{}+++\n{}", meta, body)),
        _ => Err(())
    }
}


pub fn export_to_hugo(mapping_config: HashMap<String, String>, digitus_content: &Vec<Value>) -> Vec<Result<String, String>>{
    let hugo_content: Vec<(String, Result<String,()>)> = digitus_content.iter().map(|r| (
        r.slug(),
        map_to_frontmatter(&mapping_config, &r)))
        .collect();

    let results : Vec<Result<String, String>> = hugo_content.iter().map(|(slug, content)| {
        match content {
            Ok(c) => match fs::write(filename("", &slug, ".md"), c) {
                Ok(_any) => Ok(slug.clone()),
                Err(_any) => Err(slug.clone())
            },
            _ => Err(slug.clone())
        }
    }).collect();

    results
}
