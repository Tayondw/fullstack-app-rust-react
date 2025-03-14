type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;

export interface Person {
	name: string;
	age: number;
	favourite_food: Option<string>;
}
