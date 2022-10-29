import { languages } from "monaco-editor";
import * as React from "react";
import { useState, useCallback } from "react";
import MonacoEditor from "react-monaco-editor";

export interface CodeEditorProps {
    className?: string;
    initCode?: string;
    language: string;
    onChange: (code: string) => string;
}

export const CodeEditor = (props: CodeEditorProps) => {
    const { onChange } = props;
    const [code, setCode] = useState(props.initCode ?? "");
    const onChangeMonaco = useCallback((newValue: string) => {
        const c = onChange(newValue);
        setCode(c);
    }, []);
    return (
        <div className={props.className}>
            <MonacoEditor
                height={500}
                language={props.language}
                value={code}
                onChange={onChangeMonaco}
            />
        </div>
    );
};
