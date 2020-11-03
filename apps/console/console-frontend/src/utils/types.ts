export function isObject(obj: unknown): obj is unknown {
    return (
        typeof obj === "object" &&
        obj !== null &&
        !Array.isArray(obj) &&
        !(obj instanceof RegExp) &&
        !(obj instanceof Date)
    );
}

export function isArray(value: any): value is any[] {
    return Array.isArray(value);
}
