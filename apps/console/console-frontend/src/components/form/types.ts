
export type FiledType = "string" | "number" | "enum" | "object" | "boolean";

export interface Field {
    name: string;
    type: FiledType;
    label: string;
    required: boolean;
    selectOptions?: SelectOptionItem[];
    children?: Field[];
}

export interface SelectOptionItem {
    label: string;
    value: string;
}

export interface FieldError {
    message: string;
}

/**
 * Validation Result
 *
 * example
 * -------
 *
 * ```json
 * {
 *   name: "root",
 *   errors: [],
 *   children: {
 *     id: {
 *       name: "id",
 *       errors: [ { message: "id is required!" } ],
 *     },
 *     age: {
 *       name: "age",
 *       errors: [],
 *     }
 *   }
 * }
 * ```
 *
 */
export interface ValidationResult {
    name: string;
    errors: FieldError[];
    children?: { [name: string]: ValidationResult };
}
