export interface WidgetProps {
    id: string;
    label: string;
    required: boolean;
    value?: unknown;
    selectOptions?: { [key: string]: boolean | string | number };
    onChange: (value: unknown) => void;
}
