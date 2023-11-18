// Cette fonction calcule la distance la plus courte entre un caractère spécifié et un autre caractère spécifié.
pub fn short_distance(grid: &Vec<Vec<char>>, distance: f32, play: &Vec<char>, enemy: &Vec<char>) -> f32 {
    
    let mut min_dist = distance; // Cette variable stocke la distance minimale trouvée.

    // Parcourir chaque ligne de la grille.
    for (yg, row) in grid.iter().enumerate() {
        // Parcourir chaque colonne de la ligne actuelle.
        for (xg, &cell) in row.iter().enumerate() {
            // Vérifier si le caractère actuel correspond au deuxième caractère dans `play`.
            if cell == play[1] {
                // Une autre boucle pour chaque ligne de la grille pour comparer avec `enemy`.
                for (ye, enemy_row) in grid.iter().enumerate() {
                    // Boucle sur chaque colonne de la ligne courante pour cette deuxième boucle.
                    for (xe, &enemy_cell) in enemy_row.iter().enumerate() {
                        // Vérifier si le caractère à cette position est dans `enemy`.
                        if enemy.contains(&enemy_cell) {
                            // Calcul de la distance euclidienne entre (yg, xg) et (ye, xe) en utilisant le théorème de Pythagore.
                            let dist = (((ye as f32) - (yg as f32)).powf(2.) + ((xe as f32) - (xg as f32)).powf(2.)).sqrt();
                            // Si la distance calculée est inférieure à `min_dist`, la mettre à jour.
                            min_dist = min_dist.min(dist);
                        }
                    }
                }
            }
        }
    }
    // Retourner la distance minimale trouvée.
    min_dist
}
