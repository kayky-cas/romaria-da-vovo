use std::{io::stdin, str::FromStr, time::Instant};

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
    fn ordenar_metade(&self) -> Self {
        let metade = self.cidades.len() / 2;

        let mut c1 = self.cidades[..metade].to_vec();

        c1.sort_by(|a, b| {
            let ta = a.coordenadas.0 + a.coordenadas.1;
            let tb = b.coordenadas.0 + b.coordenadas.1;
            ta.partial_cmp(&tb).unwrap()
        });

        let mut c2 = self.cidades[metade..].to_vec();
        c2.sort_by(|a, b| {
            let ta = a.coordenadas.0 + a.coordenadas.1;
            let tb = b.coordenadas.0 + b.coordenadas.1;
            tb.partial_cmp(&ta).unwrap()
        });

        c1.append(&mut c2);

        Viagem::from(c1)
    }

    fn trocar(&self, rng: &mut ThreadRng) -> Self {
        let len = self.cidades.len();
        let mut cidades = self.cidades.clone();

        let i = rng.gen_range(0..len / 2);
        let j = len - i - 1;

        cidades.swap(i, j);

        Viagem::from(cidades)
    }
}

const TAMANHO_POPULACAO: usize = 200;

fn main() {
    assert!(TAMANHO_POPULACAO > 0);

    let mut rng = thread_rng();
    let mut populacao: Vec<Viagem> = Vec::with_capacity(TAMANHO_POPULACAO);

    let mut cidades: Vec<Cidade> = stdin()
        .lines()
        .skip(1)
        .flatten()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    for _ in 0..TAMANHO_POPULACAO {
        cidades.shuffle(&mut rng);
        let viagem = Viagem::from(cidades.clone()).ordenar_metade();
        populacao.push(viagem);
    }

    let mut last_min_dist = populacao
        .iter()
        .map(|p| p.distancia)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let instant = Instant::now();

    loop {
        let idx = rng.gen_range(0..populacao.len());
        let viagem = &populacao[idx];

        let mutacao = viagem.trocar(&mut rng).ordenar_metade();

        if mutacao.distancia < viagem.distancia {
            if mutacao.distancia < last_min_dist {
                last_min_dist = mutacao.distancia;
                println!("Distancia: {} in {:?}", last_min_dist, instant.elapsed());
            }

            populacao[idx] = mutacao;
            continue;
        }
    }
}
