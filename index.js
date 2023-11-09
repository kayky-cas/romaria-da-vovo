function getRandInRange(min, max) {
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

function shuffleArray(array) {
	for (let i = array.length - 1; i > 0; i--) {
		let j = getRandInRange(0, i);
		[array[i], array[j]] = [array[j], array[i]];
	}

	return array;
}

class City {
	constructor(name, x, y) {
		this.name = name;
		this.x = x;
		this.y = y;
	}

	distanceTo(city) {
		const dx = Math.abs(this.x - city.x);
		const dy = Math.abs(this.y - city.y);

		return Math.sqrt(
			Math.pow(dx, 2) + Math.pow(dy, 2)
		);
	}
}

class Travel {
	constructor(cities, distance = null) {
		this.cities = cities;
		this.distance = distance ?? this.calculateDistance();
	}

	calculateDistance() {
		let distance = this.cities[this.cities.length - 1].distanceTo(this.cities[0]);

		for (let i = 0; i < this.cities.length - 1; i++) {
			distance += this.cities[i].distanceTo(this.cities[i + 1]);
		}

		return distance;
	}

	changeCitiesOrder(idx1, idx2) {
		const oldCity1Dist = this.cities[idx1].distanceTo(this.cities[idx1 - 1]) + this.cities[idx1].distanceTo(this.cities[idx1 + 1]);

		const oldCity2Dist = this.cities[idx2].distanceTo(this.cities[idx2 - 1]) + this.cities[idx2].distanceTo(this.cities[idx2 + 1]);

		const newCity1Dist = this.cities[idx1].distanceTo(this.cities[idx2 - 1]) + this.cities[idx1].distanceTo(this.cities[idx2 + 1]);

		const newCity2Dist = this.cities[idx2].distanceTo(this.cities[idx1 - 1]) + this.cities[idx2].distanceTo(this.cities[idx1 + 1]);

		const distance = this.distance + newCity1Dist + newCity2Dist - oldCity1Dist - oldCity2Dist;

		if (distance < this.distance) {
			this.distance = distance;
			[this.cities[idx1], this.cities[idx2]] = [this.cities[idx2], this.cities[idx1]];
		}
	}
}
