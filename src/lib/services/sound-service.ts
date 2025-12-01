import { writable, get } from 'svelte/store';

export interface Sound {
    id: string;
    name: string;
    url: string;
}

export const sounds: Sound[] = [
    {
        id: 'birds',
        name: 'sounds.birds',
        url: '/sounds/birds.mp3'
    },
    {
        id: 'forest',
        name: 'sounds.forest',
        url: '/sounds/forest.mp3'
    },
    {
        id: 'sea',
        name: 'sounds.sea',
        url: '/sounds/sea.mp3'
    },
    {
        id: 'storm',
        name: 'sounds.storm',
        url: '/sounds/storm.mp3'
    }
];

class SoundService {
    private audio: HTMLAudioElement | null = null;

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
     * Joue un son
     */
    play(sound: Sound) {
        // Si c'est le même son qui joue déjà, ne rien faire
        const current = get(this.currentSound);
        if (current?.id === sound.id && get(this.isPlaying)) {
            return;
        }

        // Arrête le son précédent
        if (this.audio) {
            this.audio.pause();
        }

        // Crée et joue le nouveau son
        this.audio = new Audio(sound.url);
        this.audio.loop = true;
        this.audio.volume = get(this.volume);

        this.audio.play().catch(err => {
            console.error("Failed to play sound:", err);
        });

        this.currentSound.set(sound);
        this.isPlaying.set(true);
    }

    /**
     * Met en pause le son actuel
     */
    pause() {
        if (this.audio) {
            this.audio.pause();
        }
        this.isPlaying.set(false);
    }

    /**
     * Reprend la lecture
     */
    resume() {
        if (this.audio && get(this.currentSound)) {
            this.audio.play();
            this.isPlaying.set(true);
        } else {
            // Si pas de son chargé, joue le premier par défaut
            this.play(sounds[0]);
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
