use compute::export::{
    egui::{emath::Numeric, DragValue, Ui},
    nalgebra::Vector3,
};
use sci_dragger::SciDragValue;

pub mod sci_dragger;

pub fn dragger<Num: Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut Num,
    func: fn(DragValue) -> DragValue,
) {
    ui.horizontal(|ui| {
        ui.add(func(DragValue::new(value)));
        ui.label(label);
    });
}

pub fn vec3_dragger<Num: Numeric>(
    ui: &mut Ui,
    val: &mut Vector3<Num>,
    func: fn(DragValue) -> DragValue,
) {
    ui.horizontal(|ui| {
        ui.add(func(DragValue::new(&mut val[0])));
        ui.label("×");
        ui.add(func(DragValue::new(&mut val[1])));
        ui.label("×");
        ui.add(func(DragValue::new(&mut val[2])));
    });
}

pub fn sci_dragger<Num: Numeric>(ui: &mut Ui, label: &str, value: &mut Num) {
    ui.horizontal(|ui| {
        SciDragValue::new(value).show(ui);
        ui.label(label);
    });
}
