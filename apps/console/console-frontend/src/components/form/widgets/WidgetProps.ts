export interface WidgetProps {
    id: string;
    label: string;
    required: boolean;
    value?: unknown;
    isError: boolean;
    selectOptions?: SelectOptionItem[];
    onChange: (value: unknown) => void;
}

export interface SelectOptionItem {
    label: string;
    value: string;
}
