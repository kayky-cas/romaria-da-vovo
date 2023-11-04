package main

import (
	"fmt"
	"math"
	"math/rand"
	"sort"
	"time"
)

type Cidade struct {
	Nome string
	Coordenadas struct {
		X float64
		Y float64
	}
}

func (c *Cidade) Distancia(cidade *Cidade) float64 {
	dx := c.Coordenadas.X - cidade.Coordenadas.X
	dy := c.Coordenadas.Y - cidade.Coordenadas.Y

	return math.Sqrt(dx*dx + dy*dy)
}

type Viagem struct {
	Cidades []Cidade
	Distancia float64
}

func OrdenarCidades(cidades []Cidade) []Cidade {

    metade := len(cidades) / 2

    cidade := cidades[0]
    sort.SliceStable(cidades[:metade], func(i, j int) bool {
        return cidades[i].Distancia(&cidade) < cidades[j].Distancia(&cidade)
    })

    cidade = cidades[metade-1]
    sort.SliceStable(cidades[metade:], func(i, j int) bool {
        return cidades[metade + i].Distancia(&cidade) < cidades[metade + j].Distancia(&cidade)
    })

	return cidades
}

func NewViagem(cidades []Cidade) Viagem {

	distancia := cidades[0].Distancia(&cidades[len(cidades)-1])

	for i := 0; i < len(cidades)-1; i++ {
		distancia += cidades[i].Distancia(&cidades[i+1])
	}

	return Viagem{cidades, distancia}
}

func (v *Viagem) Trocar() []Cidade {
	cidades := make([]Cidade, len(v.Cidades))
	copy(cidades, v.Cidades)

	i := rand.Intn(len(cidades) / 2)
	j := len(cidades) - i - 1

	cidades[i], cidades[j] = cidades[j], cidades[i]

	return OrdenarCidades(cidades)
}

func (v *Viagem) TrocarAleatorio() []Cidade {
	cidades := make([]Cidade, len(v.Cidades))
	copy(cidades, v.Cidades)

	i := rand.Intn(len(cidades)/ 2)
	j := rand.Intn(len(cidades) / 2) + len(cidades) / 2

	cidades[i], cidades[j] = cidades[j], cidades[i]

	return OrdenarCidades(cidades)
}

func main() {
	var n int
	fmt.Scanln(&n)

	cidades := make([]Cidade, n)

	for i := 1; i < n; i++ {
		fmt.Scanln(&cidades[i].Coordenadas.X, &cidades[i].Coordenadas.Y, &cidades[i].Nome)
	}

	populacao := make([]Viagem, 100)

	min := math.MaxFloat64

	for i := 0; i < 100; i++ {
		rand.Shuffle(len(cidades), func(i, j int) {
			cidades[i], cidades[j] = cidades[j], cidades[i]
		})

		populacao[i] = NewViagem(cidades)

		if populacao[i].Distancia < min {
			min = populacao[i].Distancia
		}
	}

	start := time.Now()

	for {
		idx := rand.Intn(len(populacao))

		viagem := populacao[idx]

		viagemTroca := NewViagem(viagem.Trocar())
		viagemTrocaAleatorio := NewViagem(viagem.TrocarAleatorio())

		if viagemTroca.Distancia < viagem.Distancia {
			viagem = viagemTroca
		}

		if viagemTrocaAleatorio.Distancia < viagem.Distancia {
			viagem = viagemTrocaAleatorio
		}

		if viagem.Distancia < min {
			min = viagem.Distancia
			fmt.Printf("Melhor distancia: %f em %s\n", min, time.Since(start))
		}

		populacao[idx] = viagem
	}
}
