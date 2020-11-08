import { Field, FiledType, SelectOptionItem } from "./types";
import { JsonSchema } from "../../modules/common";
import { isArray } from "../../utils/types";

function parseType(schema: JsonSchema): FiledType {
    if (schema.oneOf) {
        return "enum";
    }
    if (!schema.type) {
        return "string";
    }
    switch (schema.type) {
        case "number":
            return "number";
        case "boolean":
            return "boolean";
        case "object":
            return "object";
        default:
            return "string";
    }
}

const parseSelectOptions = (oneOf: JsonSchema[]): SelectOptionItem[] => {
    if (!oneOf) {
        return [];
    }
    const r = oneOf
        .filter((x) => x.const)
        .map((x) => {
            return {
                label: x.const,
                value: x.const,
            } as SelectOptionItem;
        });
    return r;
};

const parseProps = (schema: JsonSchema, key: string, root?: string): Field => {
    if (!schema) {
        return null;
    }
    const targetNode = (root ? schema.properties[root] : schema) as JsonSchema;

    const result: Field = {
        name: key,
        type: parseType(targetNode),
        label: targetNode.title || key,
        selectOptions: parseSelectOptions(schema.oneOf),
        required: targetNode.required ? true : false,
    };
    if (!targetNode.type || (targetNode.type && targetNode.type !== "object")) {
        return result;
    }
    // eslint-disable-next-line @typescript-eslint/no-use-before-define
    const children = parse(targetNode);
    result.children = children || [];
    return result;
};

export const parse = (schema: JsonSchema): Field[] => {
    if (!schema) {
        return [];
    }
    const m = new Map<string, Field>();
    const properties = schema.properties;
    if (properties) {
        Object.keys(properties).forEach((key) => {
            const property = properties[key];
            const f = parseProps(property, key);
            if (f) {
                m.set(key, f);
            }
        });
    }
    const required = schema.required;
    if (isArray(required)) {
        required.forEach((x) => {
            if (m.has(x)) {
                m.get(x).required = true;
            }
        });
    }
    return [...m.values()];
};
