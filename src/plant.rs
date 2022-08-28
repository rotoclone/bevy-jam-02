use crate::*;

pub struct Plant {
    pub name: String,
    pub genes: Vec<Gene>,
}

pub struct Planters(pub Vec<Planter>);

pub enum Planter {
    Plant(Plant),
    DeadPlant(Plant),
    Seed(Seed),
    Empty,
}

pub struct Phenotype {
    pub stem_style: StemStyle,
    pub stem_color: StemColor,
    pub fruit_style: FruitStyle,
    pub fruit_color: FruitColor,
    pub intelligence: i32,
    pub pest_resistance: i32,
}

impl Plant {
    pub fn get_phenotype(&self) -> Phenotype {
        let mut intelligence = 0;
        let mut pest_resistance = 0;

        //
        // stem style
        //
        let default_stem_style_gene = Gene::new_with_stem_style(StemStyle::Curvy);
        let stem_style_gene = get_expressed_gene(
            &self.genes,
            |gene| matches!(gene.category, GeneCategory::StemStyle(_)),
            &default_stem_style_gene,
        );

        let stem_style = match stem_style_gene.category {
            GeneCategory::StemStyle(x) => x,
            _ => unreachable!("stem style gene isn't in the stem style category somehow"),
        };

        intelligence += stem_style_gene.intelligence_effect;
        pest_resistance += stem_style_gene.pest_resistance_effect;

        //
        // stem color
        //
        let default_stem_color_gene = Gene::new_with_stem_color(StemColor::Green);
        let stem_color_gene = get_expressed_gene(
            &self.genes,
            |gene| matches!(gene.category, GeneCategory::StemColor(_)),
            &default_stem_color_gene,
        );

        let stem_color = match stem_color_gene.category {
            GeneCategory::StemColor(x) => x,
            _ => unreachable!("stem color gene isn't in the stem color category somehow"),
        };

        intelligence += stem_color_gene.intelligence_effect;
        pest_resistance += stem_color_gene.pest_resistance_effect;

        //
        // fruit style
        //
        let default_fruit_style_gene = Gene::new_with_fruit_style(FruitStyle::Circle);
        let fruit_style_gene = get_expressed_gene(
            &self.genes,
            |gene| matches!(gene.category, GeneCategory::FruitStyle(_)),
            &default_fruit_style_gene,
        );

        let fruit_style = match fruit_style_gene.category {
            GeneCategory::FruitStyle(x) => x,
            _ => unreachable!("fruit style gene isn't in the fruit style category somehow"),
        };

        intelligence += fruit_style_gene.intelligence_effect;
        pest_resistance += fruit_style_gene.pest_resistance_effect;

        //
        // fruit color
        //
        let default_fruit_color_gene = Gene::new_with_fruit_color(FruitColor::Red);
        let fruit_color_gene = get_expressed_gene(
            &self.genes,
            |gene| matches!(gene.category, GeneCategory::FruitColor(_)),
            &default_fruit_color_gene,
        );

        let fruit_color = match fruit_color_gene.category {
            GeneCategory::FruitColor(x) => x,
            _ => unreachable!("fruit color gene isn't in the fruit color category somehow"),
        };

        intelligence += fruit_color_gene.intelligence_effect;
        pest_resistance += fruit_color_gene.pest_resistance_effect;

        Phenotype {
            stem_style,
            stem_color,
            fruit_style,
            fruit_color,
            intelligence,
            pest_resistance,
        }
    }
}

/// Splices together the genes of 2 plants
pub fn splice_plants(plant_1: &Plant, plant_2: &Plant) -> Seed {
    Seed {
        parent_name_1: plant_1.name.clone(),
        parent_name_2: plant_2.name.clone(),
        genes: vec![], //TODO
    }
}

fn get_expressed_gene<'a, F>(
    genes: &'a [Gene],
    category_filter: F,
    default_gene: &'a Gene,
) -> &'a Gene
where
    F: Fn(&Gene) -> bool,
{
    let dominant_genes = get_matching_genes(genes, GeneDominance::Dominant, &category_filter);
    let recessive_genes = get_matching_genes(genes, GeneDominance::Recessive, &category_filter);

    dominant_genes
        .first()
        .or_else(|| recessive_genes.first())
        .unwrap_or(&default_gene)
}

fn get_matching_genes<'a, F>(
    genes: &'a [Gene],
    dominance: GeneDominance,
    additional_filter: &F,
) -> Vec<&'a Gene>
where
    F: Fn(&Gene) -> bool,
{
    genes
        .iter()
        .filter(|gene| gene.dominance == dominance && additional_filter(gene))
        .collect::<Vec<&Gene>>()
}

pub struct Seed {
    pub parent_name_1: String,
    pub parent_name_2: String,
    pub genes: Vec<Gene>,
}

pub struct Seeds(pub Vec<Seed>);

pub struct Gene {
    category: GeneCategory,
    dominance: GeneDominance,
    intelligence_effect: i32,
    pest_resistance_effect: i32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GeneDominance {
    Dominant,
    Recessive,
}

#[derive(Hash, PartialEq, Eq)]
pub enum GeneCategory {
    StemStyle(StemStyle),
    StemColor(StemColor),
    FruitStyle(FruitStyle),
    FruitColor(FruitColor),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum StemStyle {
    Curvy,
    Loopy,
    Angular,
    Wiggly,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum StemColor {
    Brown,
    Green,
    Blue,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum FruitStyle {
    Circle,
    Square,
    Triangle,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum FruitColor {
    Red,
    Purple,
    Yellow,
}

impl Gene {
    pub fn new_with_category(category: GeneCategory) -> Gene {
        match category {
            GeneCategory::StemStyle(x) => Gene::new_with_stem_style(x),
            GeneCategory::StemColor(x) => Gene::new_with_stem_color(x),
            GeneCategory::FruitStyle(x) => Gene::new_with_fruit_style(x),
            GeneCategory::FruitColor(x) => Gene::new_with_fruit_color(x),
        }
    }

    fn new_with_stem_style(style: StemStyle) -> Gene {
        let dominance;
        let intelligence_effect;
        let pest_resistance_effect;
        match style {
            StemStyle::Curvy => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = -1;
                pest_resistance_effect = 1;
            }
            StemStyle::Loopy => {
                dominance = GeneDominance::Recessive;
                intelligence_effect = 4;
                pest_resistance_effect = -1;
            }
            StemStyle::Angular => {
                dominance = GeneDominance::Recessive;
                intelligence_effect = 5;
                pest_resistance_effect = -1;
            }
            StemStyle::Wiggly => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = 1;
                pest_resistance_effect = 3;
            }
        }

        Gene {
            category: GeneCategory::StemStyle(style),
            dominance,
            intelligence_effect,
            pest_resistance_effect,
        }
    }

    fn new_with_stem_color(color: StemColor) -> Gene {
        let dominance;
        let intelligence_effect;
        let pest_resistance_effect;
        match color {
            StemColor::Brown => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = -1;
                pest_resistance_effect = 4;
            }
            StemColor::Green => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = -1;
                pest_resistance_effect = 2;
            }
            StemColor::Blue => {
                dominance = GeneDominance::Recessive;
                intelligence_effect = 5;
                pest_resistance_effect = -1;
            }
        }

        Gene {
            category: GeneCategory::StemColor(color),
            dominance,
            intelligence_effect,
            pest_resistance_effect,
        }
    }

    fn new_with_fruit_style(style: FruitStyle) -> Gene {
        let dominance;
        let intelligence_effect;
        let pest_resistance_effect;
        match style {
            FruitStyle::Circle => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = -1;
                pest_resistance_effect = 3;
            }
            FruitStyle::Square => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = 3;
                pest_resistance_effect = -1;
            }
            FruitStyle::Triangle => {
                dominance = GeneDominance::Recessive;
                intelligence_effect = 4;
                pest_resistance_effect = 1;
            }
        }

        Gene {
            category: GeneCategory::FruitStyle(style),
            dominance,
            intelligence_effect,
            pest_resistance_effect,
        }
    }

    fn new_with_fruit_color(color: FruitColor) -> Gene {
        let dominance;
        let intelligence_effect;
        let pest_resistance_effect;
        match color {
            FruitColor::Red => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = -1;
                pest_resistance_effect = 3;
            }
            FruitColor::Purple => {
                dominance = GeneDominance::Dominant;
                intelligence_effect = 2;
                pest_resistance_effect = 1;
            }
            FruitColor::Yellow => {
                dominance = GeneDominance::Recessive;
                intelligence_effect = 4;
                pest_resistance_effect = -3;
            }
        }

        Gene {
            category: GeneCategory::FruitColor(color),
            dominance,
            intelligence_effect,
            pest_resistance_effect,
        }
    }
}
