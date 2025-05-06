use scraper::{ElementRef, Html, Selector};

pub trait ElementSelector<'a> {
    fn select_text(&self, selector: &Selector) -> anyhow::Result<String>;

    fn select_attr_text(&self, selector: &Selector, attr: &str) -> anyhow::Result<String>;
}

impl<'a> ElementSelector<'a> for ElementRef<'a> {
    fn select_text(&self, selector: &Selector) -> anyhow::Result<String> {
        match self
            .select(selector)
            .next() {
            None => Err(anyhow::Error::msg("ElementSelector::select_text for ElementRef occurs error, selector not found")),
            Some(element_ref) => Ok(element_ref.text().collect::<String>())
        }
    }

    fn select_attr_text(&self, selector: &Selector, attr: &str) -> anyhow::Result<String> {
        match self
            .select(selector)
            .next() {
            None => Err(anyhow::Error::msg("ElementSelector::select_attr_text for ElementRef occurs error, selector not found")),
            Some(element_ref) => {
                match element_ref.attr(attr) {
                    None => Err(anyhow::Error::msg(format!("ElementSelector::select_attr_text for ElementRef occurs error, attr not found...{}", attr))),
                    Some(attr_str) => Ok(attr_str.into())
                }
            }
        }
    }
}

impl<'a> ElementSelector<'a> for Html {
    fn select_text(&self, selector: &Selector) -> anyhow::Result<String> {
        match self
            .select(selector)
            .next() {
            None => Err(anyhow::Error::msg("ElementSelector::select_text for Html occurs error, selector not found")),
            Some(element_ref) => Ok(element_ref.text().collect::<String>())
        }
    }

    fn select_attr_text(&self, selector: &Selector, attr: &str) -> anyhow::Result<String> {
        match self
            .select(selector)
            .next() {
            None => Err(anyhow::Error::msg("ElementSelector::select_text for Html occurs error, selector not found")),
            Some(element_ref) => {
                match element_ref.attr(attr) {
                    None => Err(anyhow::Error::msg(format!("ElementSelector::select_text for Html occurs error, attr not found...{}", attr))),
                    Some(attr_str) => Ok(attr_str.into())
                }
            }
        }
    }
}