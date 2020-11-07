#[cfg(test)]
mod tests {
    use expand::expand;
    use byte_match::bu8;
    #[test]
    fn it_works() {
        let strr = String::from("jojo2");

        match strr.as_bytes() {
            bu8!["a" x @ .. "b"] => {}
            bu8!["c" x @ .. "d"] => {}
            bu8!["a" x @ .. "b"] | bu8!["c" x @ .. "d"] => {}
            [b'a', x @ .., b'b'] | [b'c', x @ .., b'd'] => {}
            expand!([b'a', x @ .., b'b']) | expand!([b'c', x @ .., b'd']) => {}
            _ => {}
        }
    }
}
