import * as React from "react";
import { useState, useCallback } from "react";
import MonacoEditor from "react-monaco-editor";

export interface CodeEditorProps {
    className?: string,
    initCode?: string
}

export const CodeEditor = (props: CodeEditorProps) => {
    const [code, setCode] = useState(props.initCode ?? "");
    const onChange = useCallback((newValue: string) => {
        setCode(newValue);
    }, []);
    const onDidMount = useCallback((editor) => {
        console.log("editor did mound");
    }, []);
    return (
        <div className={props.className}>
            <MonacoEditor
                height={500}
                language="yaml"
                value={code}
                onChange={onChange}
                editorDidMount={onDidMount}
            />
        </div>
    );
};
