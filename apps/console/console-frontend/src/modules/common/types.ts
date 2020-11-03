export interface JsonSchema {
    $id?: string;
    $ref?: string;
    $schema?: string;

    type?: string;
    enum?: string[];

    const?: string;
    oneOf?: JsonSchema[];

    required?: string[];
    properties?: { [key: string]: JsonSchema };

    definitions?: { [key: string]: JsonSchema };

    title?: string;
    description?: string;
}
