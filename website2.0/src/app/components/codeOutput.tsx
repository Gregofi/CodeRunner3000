export function CodeOutput({ code }: { code: string }) {
    return (
        <div className="overflow-auto border font-mono p-2 border-gray-300 grow">
            <pre>
                <code>{code}</code>
            </pre>
        </div>
    );
}
