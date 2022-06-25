use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, Entry};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("null.daknig.pay_split")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(activate);

    // Run the application
    app.run();
}

#[derive(Debug, PartialEq)]
struct Transaction {
    from: String,
    to: String,
    sum: f32,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} pays {:.2} to {}", self.from, self.sum, self.to)
    }
}

struct TransactionList {
    amounts: Vec<Entry>,
    names: Vec<Entry>,
    widget: gtk::ListBox,
}

impl TransactionList {
    /// split the pay, display result
    fn split_pay(&self) -> Vec<Transaction> {
        let mut people = HashMap::new();
        let mut total: f32 = 0f32;
        for (name, sum) in self.names.iter().zip(&self.amounts) {
            let sum: f32 = sum.text().parse().unwrap_or(0f32);
            total += sum;
            *people.entry(name.text()).or_insert(0f32) += sum;
        }

        let empty = glib::GString::from("");
        if let Some(v) = people.get("") {
            total -= v;
            let empty = empty.as_str();
            people.remove(empty);
            // also go around and make the empty entry boxes red!
        }

        if total == 0f32 {
            return vec![];
        }

        #[allow(clippy::cast_precision_loss)]
        let avg: f32 = total / (people.len() as f32);
        let mut ret_val: Vec<Transaction> = vec![];

        let mut sums: Vec<_> = people
            .drain()
            .map(|(k, v)| (k, v - avg))
            .filter(|(_, v)| v.abs() >= 0.01) // EPSILON
            .collect::<Vec<_>>();

        sums.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Less));

        let mut sums = VecDeque::from(sums);

        while sums.len() > 1 {
            let sum_to_pay = sums
                .front()
                .unwrap()
                .1
                .abs()
                .min(sums.back().unwrap().1.abs());

            ret_val.push(Transaction {
                from: sums.front().unwrap().0.to_string(),
                to: sums.back().unwrap().0.to_string(),
                sum: sum_to_pay,
            });

            sums.front_mut().unwrap().1 += sum_to_pay;
            sums.back_mut().unwrap().1 -= sum_to_pay;

            if sums.front().unwrap().1.abs() < 0.01 {
                // EPSILON
                sums.pop_front();
            }
            if sums.back().unwrap().1.abs() < 0.01 {
                // EPSILON
                sums.pop_back();
            }
        }

        ret_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_avg_algo() {
        let amounts: Vec<f32> = vec![12.0, 0.0];
        let total: f32 = amounts.iter().sum();
        let avg = total / (amounts.len() as f32);
        assert_eq!(avg, 6.);

        assert_eq!(
            amounts.iter().map(|x| x - avg).collect::<Vec<_>>(),
            vec![6., -6.]
        )
    }
    #[test]
    fn split_algo() {
        gtk::init().ok().unwrap();
        let names = vec!["A", "B"];
        let amounts = vec!["12", ""];

        fn string_to_entry(s: &&str) -> Entry {
            Entry::builder()
                .buffer(&gtk::EntryBuffer::new(Some(s)))
                .build()
        }

        let t = TransactionList {
            amounts: amounts.iter().map(string_to_entry).collect(),
            names: names.iter().map(string_to_entry).collect(),
            widget: gtk::ListBox::new(),
        };

        println!("{:?}", t.split_pay());
        assert_eq!(
            t.split_pay(),
            vec![Transaction {
                from: "B".to_string(),
                to: "A".to_string(),
                sum: 6.0
            }]
        );
    }
}

impl TransactionList {
    /// add a new row if the last row is not empty
    fn add_empty_row(&mut self) {
        // construct the Entrys
        self.amounts.push(
            Entry::builder()
                .placeholder_text("0")
                .max_width_chars(6)
                .input_purpose(gtk::InputPurpose::Number)
                .input_hints(gtk::InputHints::PRIVATE)
                .xalign(1.)
                .build(),
        );

        self.names.push(
            Entry::builder()
                .placeholder_text("name")
                .max_width_chars(30)
                .build(),
        );

        // put them in a Box
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        row.append(self.names.last().unwrap());
        row.append(self.amounts.last().unwrap());

        // attach them to the widget
        self.widget.append(&row);
    }
}

fn activate(app: &Application) {
    // Init `gtk::Builder` from file
    let builder = gtk::Builder::from_string(include_str!("gui4.xml"));

    // Get window and button from `gtk::Builder`
    let window: gtk::Window = builder
        .object("window")
        .expect("Could not get object `window` from builder.");

    // Set application
    window.set_application(Some(app));

    // OK IF RUST WANTS ME TO DO IT WITH HECKING CLOSURES...
    let add_button: gtk::Button = builder
        .object("add_button")
        .expect("Could not get object `add_button` from builder.");

    let list = Rc::new(RefCell::new(TransactionList {
        amounts: vec![],
        names: vec![],
        widget: builder
            .object("pay_list")
            .expect("Could not get object `pay_list` from builder."),
    }));

    let split_button: gtk::Button = builder
        .object("split_button")
        .expect("Could not get object `pay_list` from builder.");

    let l1 = Rc::clone(&list);
    split_button.connect_clicked(move |_| {
        println!("{:?}", l1.borrow().split_pay());
    });

    add_button.connect_clicked(move |_| list.borrow_mut().add_empty_row());

    // add_button.connect
    // glib::clone!(@strong pay_list => move |_| {

    // })

    window.show();
}
