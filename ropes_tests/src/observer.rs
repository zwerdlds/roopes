#[cfg(test)]
mod tests
{
    use enclose::enclose;
    use ropes::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };
    #[derive(Hash, PartialEq, Eq)]
    enum TestCommands
    {
        HasRun,
        HasRunToggle,
        HasRunTwo,
    }

    #[test]
    fn simple_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = command::Lambda::new(move || {
            {
                (*has_run_ext.borrow_mut()) = true;
            }
        });

        let hc: observing_command::ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        hs.notify();

        assert!((*has_run.borrow()));
    }

    #[test]
    fn toggle_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_toggle = Rc::new(RefCell::new(false));
        let lc = command::Lambda::new(enclose!(
            (has_run_toggle) move || {
                {
                    let tgl = *has_run_toggle.borrow();
                    (*has_run_toggle.borrow_mut()) = !tgl;
                }
                .into()
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRunToggle).into();

        assert!(!(*has_run_toggle.borrow()));
        hs.attach(hc);

        hs.notify();
        assert!((*has_run_toggle.borrow()));

        hs.notify();
        assert!(!(*has_run_toggle.borrow()));

        hs.notify();
        assert!((*has_run_toggle.borrow()));
    }

    #[test]
    fn multiple_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_1) move || {
                (*has_run_1.borrow_mut()) = true;
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);
        assert!(!(*has_run_1.borrow()));

        hs.notify();
        assert!((*has_run_1.borrow()));

        let mut hs: observer::HashSubject<ObservingCommand<_>> =
            Default::default();

        let has_run_2 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_2) move || {
                (*has_run_2.borrow_mut()) = true;
            }
        ));
        let hc = command::Hashable::new(lc, TestCommands::HasRunTwo);

        hs.attach(hc.into());

        assert!((*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!((*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }

    #[test]
    fn overwrite_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_1) move || {
                (*has_run_1.borrow_mut()) = true;
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        let mut hs: observer::HashSubject<ObservingCommand<_>> =
            observer::HashSubject::default();

        let has_run_2 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_2) move || {
                (*has_run_2.borrow_mut()) = true;
            }
        ));

        let hc = command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        assert!(!(*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!(!(*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }
    #[test]
    fn simple_vector_subject_notify()
    {
        let mut vs = observer::VecSubject::default();

        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc: ObservingCommand<_> = command::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        })
        .into();

        vs.attach(lc);

        vs.notify();
        assert!((*has_run.borrow()));
    }

    #[test]
    fn toggle_vector_subject_notify()
    {
        let mut vs = observer::VecSubject::default();

        let has_run_toggle = Rc::new(RefCell::new(false));
        let has_run_toggle_ext = has_run_toggle.clone();

        let lc: ObservingCommand<_> = command::Lambda::new(move || {
            let tgl = *has_run_toggle_ext.borrow();

            (*has_run_toggle_ext.borrow_mut()) = !tgl;
        })
        .into();

        vs.attach(lc);

        assert!(!(*has_run_toggle.borrow()));

        vs.notify();
        assert!((*has_run_toggle.borrow()));

        vs.notify();
        assert!(!(*has_run_toggle.borrow()));

        vs.notify();
        assert!((*has_run_toggle.borrow()));
    }

    #[test]
    fn multiple_vector_subject_notify()
    {
        let mut vs = observer::VecSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));
        let has_run_1_ext = has_run_1.clone();

        let lc: ObservingCommand<_> = command::Lambda::new(move || {
            (*has_run_1_ext.borrow_mut()) = true;
        })
        .into();

        vs.attach(lc);

        assert!(!(*has_run_1.borrow()));

        vs.notify();
        assert!((*has_run_1.borrow()));

        let mut vs = observer::VecSubject::default();

        let has_run_2 = Rc::new(RefCell::new(false));
        let has_run_2_ext = has_run_2.clone();

        let lc: ObservingCommand<_> = command::Lambda::new(move || {
            (*has_run_2_ext.borrow_mut()) = true;
        })
        .into();

        vs.attach(lc);

        assert!((*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        vs.notify();

        assert!((*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }
}
