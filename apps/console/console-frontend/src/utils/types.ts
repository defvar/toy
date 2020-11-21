export function isObject(obj: unknown): obj is unknown {
    return (
        typeof obj === "object" &&
        obj !== null &&
        !Array.isArray(obj) &&
        !(obj instanceof RegExp) &&
        !(obj instanceof Date)
    );
}

export function isArray(value: unknown): value is unknown[] {
    return Array.isArray(value);
}
