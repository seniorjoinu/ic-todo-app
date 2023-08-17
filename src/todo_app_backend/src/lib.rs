use std::cell::RefCell;

use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};

pub type Index = usize;

#[derive(CandidType, Deserialize, Clone)]
pub struct Element {
    title: String,
    status: Status,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum Status {
    Todo,
    Done,
}

#[derive(CandidType)]
pub enum Result {
    Ok,
    IndexOutOfBounds,
}

thread_local! {
    static STATE: RefCell<Vec<Element>> = RefCell::new(Vec::new());
}

#[update]
pub fn add_element_at(idx: Index, elem: Element) -> Result {
    STATE.with(|it| {
        let mut elements = it.borrow_mut();

        if elements.len() < idx {
            return Result::IndexOutOfBounds;
        }

        elements.insert(idx, elem);

        Result::Ok
    })
}

#[update]
pub fn remove_element_at(idx: Index) -> Result {
    STATE.with(|it| {
        let mut elements = it.borrow_mut();

        if elements.len() <= idx {
            return Result::IndexOutOfBounds;
        }

        elements.remove(idx);

        Result::Ok
    })
}

#[update]
pub fn update_element_at(idx: Index, updated_elem: Element) -> Result {
    STATE.with(|it| {
        let mut elements = it.borrow_mut();

        if elements.len() <= idx {
            return Result::IndexOutOfBounds;
        }

        elements[idx] = updated_elem;

        Result::Ok
    })
}

#[query]
pub fn list_all() -> Vec<Element> {
    STATE.with(|it| it.borrow().clone())
}

#[cfg(test)]
mod tests {
    // чтобы импорты работали, каждое импортируемое имя нужно пометить словом pub
    use crate::{add_element_at, list_all, remove_element_at, update_element_at, Element, Result};

    #[test]
    fn works_fine() {
        // создаём 5 элементов

        let elem_1 = Element {
            title: "First".to_string(),
            status: crate::Status::Todo,
        };
        let elem_2 = Element {
            title: "Second".to_string(),
            status: crate::Status::Todo,
        };
        let elem_3 = Element {
            title: "Third".to_string(),
            status: crate::Status::Todo,
        };
        let elem_4 = Element {
            title: "Forth".to_string(),
            status: crate::Status::Todo,
        };
        let elem_5 = Element {
            title: "Fifth".to_string(),
            status: crate::Status::Todo,
        };

        // вставляем их последовательно в список

        let res = add_element_at(0, elem_1.clone());
        assert!(matches!(res, Result::Ok));

        let res = add_element_at(1, elem_2.clone());
        assert!(matches!(res, Result::Ok));

        let res = add_element_at(2, elem_3.clone());
        assert!(matches!(res, Result::Ok));

        let res = add_element_at(3, elem_4.clone());
        assert!(matches!(res, Result::Ok));

        let res = add_element_at(4, elem_5.clone());
        assert!(matches!(res, Result::Ok));

        // проверяем, что все элементы на своих местах

        let list = list_all();

        assert_eq!(list.len(), 5);
        assert_eq!(list[0].title, elem_1.title);
        assert_eq!(list[1].title, elem_2.title);
        assert_eq!(list[2].title, elem_3.title);
        assert_eq!(list[3].title, elem_4.title);
        assert_eq!(list[4].title, elem_5.title);

        // удаляем четверный элемент

        let res = remove_element_at(3);
        assert!(matches!(res, Result::Ok));

        // проверяем, что остальные элементы на своих местах

        let list = list_all();

        assert_eq!(list.len(), 4);
        assert_eq!(list[0].title, elem_1.title);
        assert_eq!(list[1].title, elem_2.title);
        assert_eq!(list[2].title, elem_3.title);
        assert_eq!(list[3].title, elem_5.title);

        // вставляем новый элемент на место старого

        let res = add_element_at(3, elem_3.clone());
        assert!(matches!(res, Result::Ok));

        // обновляем элемент так, чтобы его содержимое соответствовало четвертому элементу

        let res = update_element_at(3, elem_4.clone());
        assert!(matches!(res, Result::Ok));

        // проверяем, что все элементы снова вставлены в список в оригинальном порядке

        let list = list_all();

        assert_eq!(list.len(), 5);
        assert_eq!(list[0].title, elem_1.title);
        assert_eq!(list[1].title, elem_2.title);
        assert_eq!(list[2].title, elem_3.title);
        assert_eq!(list[3].title, elem_4.title);
        assert_eq!(list[4].title, elem_5.title);
    }
}
