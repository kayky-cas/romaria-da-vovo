use std::{env, io::stdin, str::FromStr, time::Instant};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
struct Cidade {
    nome: &'static str,
    coordenadas: (f64, f64),
}

impl Cidade {
    fn distancia(&self, outra: &Cidade) -> f64 {
        let dx = self.coordenadas.0 - outra.coordenadas.0;
        let dy = self.coordenadas.1 - outra.coordenadas.1;
        (dx * dx + dy * dy).sqrt()
    }
}

impl FromStr for Cidade {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 65.6478 68.3254 Cid1000
        let mut partes = s.split_whitespace();

        let x: f64 = partes
            .next()
            .expect("Nao foi encontrado um X")
            .parse()
            .expect("X deve ser um Double");

        let y: f64 = partes
            .next()
            .expect("Nao foi encontrado um Y")
            .parse()
            .expect("Y deve ser um Double");

        let cidade = partes
            .next()
            .expect("Nao foi possivel encontrar o nome da Cidade");

        let cidade = Box::leak(cidade.to_owned().into_boxed_str());

        Ok(Self {
            nome: cidade,
            coordenadas: (x, y),
        })
    }
}

#[derive(Debug)]
struct Viagem {
    distancia: f64,
    cidades: Vec<Cidade>,
}

impl From<Vec<Cidade>> for Viagem {
    fn from(cidades: Vec<Cidade>) -> Self {
        let mut distancia_total = cidades[0].distancia(&cidades[cidades.len() - 1]);

        for i in 0..cidades.len() - 1 {
            distancia_total += cidades[i].distancia(&cidades[i + 1]);
        }

        Viagem {
            distancia: distancia_total,
            cidades,
        }
    }
}

impl Viagem {
    fn ordenar_metade(mut cidades: Vec<Cidade>) -> Self {
        let metade = cidades.len() / 2;

        let cidade = cidades[0].clone();
        cidades[1..metade].sort_by(|a, b| {
            a.distancia(&cidade)
                .partial_cmp(&b.distancia(&cidade))
                .unwrap()
        });

        let cidade = cidades[metade - 1].clone();
        cidades[metade..].sort_by(|a, b| {
            a.distancia(&cidade)
                .partial_cmp(&b.distancia(&cidade))
                .unwrap()
        });

        Viagem::from(cidades)
    }

    fn trocar(&self, rng: &mut ThreadRng) -> Vec<Cidade> {
        let len = self.cidades.len();
        let mut cidades = self.cidades.clone();

        let i = rng.gen_range(0..len / 2);
        let j = len - i - 1;

        cidades.swap(i, j);

        cidades
    }

    fn trocar_rand(&self, rng: &mut ThreadRng) -> Vec<Cidade> {
        let len = self.cidades.len();
        let mut cidades = self.cidades.clone();

        let i = rng.gen_range(0..len / 2);
        let j = rng.gen_range(len / 2..len);

        cidades.swap(i, j);

        cidades
    }
}

fn main() {
    let tamanho_populacao = env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(100);

    let mut rng = thread_rng();
    let mut populacao: Vec<Viagem> = Vec::with_capacity(tamanho_populacao);

    let mut cidades: Vec<Cidade> = stdin()
        .lines()
        .skip(1)
        .flatten()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    let mut min_dist = f64::MAX;

    for _ in 0..tamanho_populacao {
        cidades.shuffle(&mut rng);
        let viagem = Viagem::ordenar_metade(cidades.clone());

        if viagem.distancia < min_dist {
            min_dist = viagem.distancia;
        }

        populacao.push(viagem);
    }

    let instant = Instant::now();

    loop {
        let idx = rng.gen_range(0..populacao.len());
        let viagem = &populacao[idx];

        let mutacao = [
            Viagem::ordenar_metade(viagem.trocar(&mut rng)),
            Viagem::ordenar_metade(viagem.trocar_rand(&mut rng)),
        ]
        .into_iter()
        .min_by(|a, b| a.distancia.partial_cmp(&b.distancia).unwrap())
        .unwrap();

        if mutacao.distancia < viagem.distancia {
            if mutacao.distancia < min_dist {
                min_dist = mutacao.distancia;
                println!("Distancia: {} em {:?}", min_dist, instant.elapsed());
            }

            populacao[idx] = mutacao;
        }
    }
}
