use serde::{Deserialize, Serialize};

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, ItemStyle, Tooltip},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Boxplot {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hover_animation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<usize>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Default for Boxplot {
    fn default() -> Self {
        Self::new()
    }
}

impl Boxplot {
    pub fn new() -> Self {
        Boxplot {
            type_: String::from("boxplot"),
            id: None,
            name: None,
            coordinate_system: None,
            color_by: None,
            legend_hover_link: None,
            hover_animation: None,
            dataset_index: None,
            tooltip: None,
            data: vec![],
            item_style: None,
            z: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn hover_animation(mut self, hover_animation: bool) -> Self {
        self.hover_animation = Some(hover_animation);
        self
    }

    pub fn dataset_index(mut self, dataset_index: u64) -> Self {
        self.dataset_index = Some(dataset_index);
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }

    pub fn z(mut self, z: usize) -> Self {
        self.z = Some(z);
        self
    }
}
