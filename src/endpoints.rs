use std::string::ParseError;
use reqwest::Url;

pub mod xrefs;

const BASE_URL: &str = "https://rest.ensembl.org";

trait Request<B> {
    fn url(&self) -> Result<Url, ParseError>;
    fn parse(&self, );
}

trait GetRequest<B> : Request<B> {

}

trait PostRequest<B>: Request<B> {

}