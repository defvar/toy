export function isObject(obj: unknown): obj is unknown {
    return (
        typeof obj === "object" &&
        obj !== null &&
        !Array.isArray(obj) &&
        !(obj instanceof RegExp) &&
        !(obj instanceof Date)
    );
}
