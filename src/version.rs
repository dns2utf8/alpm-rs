//split EVR into release, version and release
pub fn parse_evr<'a>(s: &'a String) -> Result<(Option<&'a str>, &'a str, Option<&'a str>), ()>{

    //ranges for the parts of the string
    //the second value of the tuple is the index of the last char
    let mut epoche: Option<(&str, usize)> = None;
    let mut version: Option<(&str, usize)> = None;
    //doesnt need an endpoint as its the last part
    let mut release: Option<&str> = None;

    for c in s.char_indices(){
        //set epoche (if available)
        if epoche.is_none() && c.1 == ':'{
            if c.0 > 0{
                epoche = Some((&s[0usize..c.0 as usize], c.0));
            }
        }
        //set version
        else if version.is_none() && c.1 == '-'{
            if c.0 > 0{
                if let Some(ep) = epoche.clone(){
                    version = Some((&s[ep.1+1..c.0 as usize], c.0));
                }
                //no epoche
                else{
                    version = Some((&s[0usize..c.0 as usize], c.0));
                }
            }
        }
    }

    //set release
    if let Some(v) = version.clone(){
        if s.len() >= v.1{
            release = Some(&s[v.1+1..]);
        }
    }
    //set version (if there is no release)
    //it doesnt matter to which value version.1 is set here
    else{
        if let Some(ep) = epoche.clone(){
            version = Some((&s[ep.1+1..], 0));
        }
        else{
            version = Some((&s[0usize..], 0));
        }
    }


    //return the result
    if let Some(v) = version{
        let e: Option<&str> = match epoche{ Some(e) => Some(e.0), None => None };

        Ok((e, v.0, release))
    }
    else{
        Err(())
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn parse_evr_test(){
        use super::parse_evr;

        let s = "2:643.2b-43".to_owned();
        let tup = parse_evr(&s).unwrap();

        assert!(tup.0.is_some());
        assert!(tup.2.is_some());
        assert_eq!(format!("{}:{}-{}", tup.0.unwrap(), tup.1, tup.2.unwrap()), s);


        let s = "643.2b".to_owned();
        let tup = parse_evr(&s).unwrap();

        assert!(tup.0.is_none());
        assert!(tup.2.is_none());
        assert_eq!(format!("{}", tup.1), s);
    }
}
