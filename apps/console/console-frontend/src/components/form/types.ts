export interface Field {
    name: string;
    type: "string" | "number" | "object";
    label: string;
    required: boolean;
    children?: Field[];
}
