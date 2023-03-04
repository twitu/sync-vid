pub(crate) mod youtube;
pub(crate) mod player_interface;


pub fn get_player() -> Result<impl player_interface::PlayerInterface, String> 
{
    let window_url = web_sys::window()
                                                .ok_or("Global window does not exist".to_owned())
                                                .map(|w| w.location())
                                                .and_then(|l| 
                                                    l.host().map_err(|e|
                                                        e.as_string().unwrap_or("Can't get host url from location".to_owned())
                                                    )
                                                )?;
    
    match window_url.as_ref() {
        "www.youtube.com" => Ok(youtube::YouTube::new()),
        _ => Err(String::from("Unsupported Player"))
    }
}