#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityParty {
    id: Option<String>,
    size: Option<(usize, usize)>,
}
