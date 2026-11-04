import { writable, get } from 'svelte/store';
import { createAudio } from './audio-source';

export interface Sound {
	id: string;
	name: string;
	url: string;
}

export const sounds: Sound[] = [
	{
		id: 'birds',
		name: 'sounds.birds',
		url: '/sounds/birds.ogg'
	},
	{
		id: 'forest',
		name: 'sounds.forest',
		url: '/sounds/forest.ogg'
	},
	{
		id: 'sea',
		name: 'sounds.sea',
		url: '/sounds/sea.ogg'
	},
	{
		id: 'storm',
		name: 'sounds.storm',
		url: '/sounds/storm.ogg'
	}
];

class SoundService {
	private audio: HTMLAudioElement | null = null;
	// Incrémenté à chaque action : permet d'ignorer un chargement devenu obsolète
	private request = 0;

	// Stores
	isPlaying = writable(false);
	currentSound = writable<Sound | null>(null);
	volume = writable(0.5);

	constructor() {
		// Met à jour le volume quand le store change
		this.volume.subscribe((v) => {
			if (this.audio) {
				this.audio.volume = v;
			}
		});
	}

	/**
	 * Joue un son (en boucle)
	 */
	async play(sound: Sound): Promise<void> {
		// Si c'est le même son qui joue déjà, ne rien faire
		const current = get(this.currentSound);
		if (current?.id === sound.id && get(this.isPlaying)) {
			return;
		}

		// Arrête le son précédent
		this.audio?.pause();
		this.audio = null;

		const request = ++this.request;
		this.currentSound.set(sound);
		this.isPlaying.set(true);

		try {
			const audio = await createAudio(sound.url);
			// Un autre son a été choisi (ou pause) pendant le chargement
			if (request !== this.request) return;
			audio.loop = true;
			audio.volume = get(this.volume);
			this.audio = audio;
			await audio.play();
		} catch (err) {
			if (request !== this.request) return;
			console.error('Failed to play sound:', err);
			this.isPlaying.set(false);
		}
	}

	/**
	 * Met en pause le son actuel
	 */
	pause() {
		this.request++;
		this.audio?.pause();
		this.isPlaying.set(false);
	}

	/**
	 * Reprend la lecture
	 */
	resume() {
		const current = get(this.currentSound);
		if (this.audio && current) {
			const request = ++this.request;
			this.isPlaying.set(true);
			this.audio.play().catch((err) => {
				if (request !== this.request) return;
				console.error('Failed to resume sound:', err);
				this.isPlaying.set(false);
			});
		} else {
			// Si pas de son chargé, rejoue le dernier choisi (ou le premier par défaut)
			void this.play(current ?? sounds[0]);
		}
	}

	/**
	 * Bascule entre lecture et pause
	 */
	toggle() {
		if (get(this.isPlaying)) {
			this.pause();
		} else {
			this.resume();
		}
	}

	/**
	 * Change le volume (0.0 à 1.0)
	 */
	setVolume(val: number) {
		this.volume.set(Math.max(0, Math.min(1, val)));
	}
}

export const soundService = new SoundService();
