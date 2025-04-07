import { writable } from 'svelte/store';

export interface Tile {
    height: number;
    moisture: number;
    biome: BiomeType;
}

export type WorldData = Tile[][];

export enum BiomeType {
    Ocean = "Ocean",
    Beach = "Beach",
    Plains = "Plains",
    Forest = "Forest",
    Mountain = "Mountain",
    Desert = "Desert",
}

export const world = writable<WorldData | undefined>(undefined);

export function setWorld(newWorldData: WorldData) {
    world.set(newWorldData);
}

export function resetWorld() {
    world.set(undefined);
}