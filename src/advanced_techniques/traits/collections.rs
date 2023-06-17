#[derive(Debug)]
struct Person {
    age: usize,
}

struct Adults(Vec<Person>);

impl FromIterator<Person> for Adults {
    fn from_iter<T: IntoIterator<Item = Person>>(iter: T) -> Self {
        let adults = iter.into_iter().filter(|person| person.age >= 18).collect();
        Self(adults)
    }
}


impl IntoIterator for Adults {
    type Item = Person;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Adults {
    async fn collect_taxes(&self) -> u64 {
        let taxes = self.0.iter().map(|adult| adult.age * 15).sum::<usize>();
        taxes as u64
    }
}



#[cfg(test)]
#[tokio::test]
async fn test_capturing_adults() {
    let people = vec![Person { age: 10 }, Person { age: 20 }];

    let adults = Adults::from_iter(people);
    let taxes = adults.collect_taxes().await;

    for adult in adults {
        println!("Adult: {:?}", adult);
    }
}
