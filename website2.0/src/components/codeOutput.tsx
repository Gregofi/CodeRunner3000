import { ExecutionData } from "@/lib/types";
import { LinearProgress } from "@mui/material";

export function CodeOutput({ executionData }: { executionData: ExecutionData }) {

    return (
        <div className="flex flex-col h-1/3">
            {executionData.pending && <LinearProgress />}
            <div className="overflow-auto border font-mono p-2 border-gray-300 grow">
                <pre>
                    <code>{executionData.result?.stdout}</code>
                </pre>
            </div>
            <div className="overflow-auto border font-mono p-2 border-gray-300 grow">
                <pre>
                    <code>{executionData.result?.stderr}</code>
                </pre>
            </div>
        </div>
    );
}
