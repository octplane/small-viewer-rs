use nom::{IResult,not_line_ending, space, alphanumeric, multispace};

use std::str;
use std::collections::HashMap;

fn category(input: &[u8]) -> IResult<&[u8], &str> {
  let (i, (_, name, _, _)) = try_parse!(input,
    tuple!(
      tag!("["),
      map_res!(
        take_until!("]"),
        str::from_utf8
      ),
      tag!("]"),
      opt!(multispace)
    )
  );

  return IResult::Done(i, name)
}

named!(key_value    <&[u8],(&str,&str)>,
  chain!(
    key: map_res!(alphanumeric, ::std::str::from_utf8) ~
         space?                            ~
         tag!("=")                         ~
         space?                            ~
    val: map_res!(
           take_until_either!("\n;"),
           str::from_utf8
         )                                 ~
         space?                            ~
         chain!(
           tag!(";")        ~
           not_line_ending  ,
           ||{}
         ) ?                               ~
         multispace?                       ,
    ||{(key, val)}
  )
);


named!(keys_and_values_aggregator<&[u8], Vec<(&str,&str)> >, many0!(key_value));

fn keys_and_values(input:&[u8]) -> IResult<&[u8], HashMap<&str, &str> > {
  let mut h: HashMap<&str, &str> = HashMap::new();

  match keys_and_values_aggregator(input) {
    IResult::Done(i,tuple_vec) => {
      for &(k,v) in &tuple_vec {
        h.insert(k, v);
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => IResult::Error(a)
  }
}

named!(category_and_keys<&[u8],(&str,HashMap<&str,&str>)>,
  chain!(
    category: category    ~
    keys: keys_and_values ,
    move ||{(category, keys)}
  )
);

named!(categories_aggregator<&[u8], Vec<(&str, HashMap<&str,&str>)> >, many0!(category_and_keys));

pub fn categories(input: &[u8]) -> IResult<&[u8], HashMap<&str, HashMap<&str, &str> > > {
  let mut h: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

  match categories_aggregator(input) {
    IResult::Done(i,tuple_vec) => {
      for &(k,ref v) in &tuple_vec {
        h.insert(k, v.clone());
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => IResult::Error(a)
  }
}
