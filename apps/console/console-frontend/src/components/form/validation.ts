import { ValidationResult, FieldError } from "./types";

export function getChildErrors(
    v: ValidationResult,
    name: string
): FieldError[] {
    if (v && v.children && v.children[name]) {
        return v.children[name].errors;
    } else {
        return [];
    }
}

export function getValidationChild(
    v: ValidationResult,
    name: string
): ValidationResult | null {
    if (v && v.children && v.children[name]) {
        return v.children[name];
    } else {
        return null;
    }
}

export function addErrors(
    v: ValidationResult,
    property: string,
    errors: FieldError[]
): ValidationResult {
    if (!property || !v) {
        return v;
    }
    const r = { ...v };
    const path = property.split(".");
    let target: ValidationResult = r;
    for (const p of path) {
        if (!target.children) {
            target.children = {};
        }
        if (!(p in target.children)) {
            target.children[p] = {
                name: p,
                errors: [],
            };
        }
        target = target.children[p];
    }

    target.errors = target.errors.concat(errors);
    return r;
}
