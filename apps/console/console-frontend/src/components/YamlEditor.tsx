import * as React from "react";
import { useState, useCallback } from "react";
import MonacoEditor from "react-monaco-editor";

export interface YamlEditorProps {
    height?: string | number;
}

export const YamlEditor = (props: YamlEditorProps) => {
    const [code, setCode] = useState("");
    const onChange = useCallback((newValue: string) => {
        setCode(newValue);
    }, []);
    const onDidMount = useCallback((editor) => {
        console.log("did mound");
        editor.focus();
    }, []);
    const height = props.height ?? 500;
    return (
        <div>
            <MonacoEditor
                height={height}
                language="yaml"
                value={code}
                onChange={onChange}
                editorDidMount={onDidMount}
            />
        </div>
    );
};
