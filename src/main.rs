use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
// Добавление derive(Clone) уже запрещает Copy
// Потому что тип String не может быть Copy
struct Cat {
    name: String,
    age: u8,
}
// для пользовательской информации реализуем трейд Display
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cat named {}, who is {} years old", self.name, self.age)
    }
}
#[allow(dead_code)]
enum Pet {
    Dog(String, u8),
    Cat(Cat),
}
//Реализуйте преобразование между Cat и Pet и наоборот:
impl From<Cat> for Pet {
    fn from(cat: Cat) -> Pet {
        Pet::Cat(cat)
    }
}

impl TryFrom<Pet> for Cat {
    type Error = &'static str;
    fn try_from(pet: Pet) -> Result<Cat, Self::Error> {
        match pet {
            Pet::Cat(cat) => Ok(cat),
            _ => Err("Not a cat"),
        }
    }
}
// Предоставление ссылки на строковый слайс - имя кота
impl Cat {
    fn get_name(&self) -> &str {
        &self.name
    }
}
// Перегрузка операций сложения и сложения с присваиванием
impl Add<u8> for Cat {
    type Output = Cat;

    fn add(self, years: u8) -> Self::Output {
        Cat {
            name: self.name,
            age: self.age + years,
        }
    }
}

impl AddAssign<u8> for Cat {
    fn add_assign(&mut self, years: u8) {
        self.age += years;
    }
}

fn main() {
    // Создание кота
    let mut my_cat = Cat {
        name: String::from("Whiskas"),
        age: 3,
    };

    // Клонирование
    let cloned_cat = my_cat.clone();
    println!("Cloned cat: {:?}", cloned_cat);

    // Отладочная и пользовательская информация
    println!("Debug: {:?}", my_cat);
    println!("Display: {}", my_cat);

    // Преобразование в Pet и обратно
    let pet: Pet = my_cat.clone().into();
    match Cat::try_from(pet) {
        Ok(cat) => println!("Converted back to cat: {:?}", cat),
        Err(e) => println!("Error: {}", e),
    }

    // Ссылка на имя
    println!("Cat's name is: {}", my_cat.get_name());

    // Сложение и сложение с присваиванием
    let older_cat = my_cat.clone() + 2;
    println!("Older cat: {:?}", older_cat);

    my_cat += 1;
    println!("Cat after adding 1 year: {:?}", my_cat);
}
