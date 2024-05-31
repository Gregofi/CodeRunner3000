'use server'

import { CurrentChoice, ExecutorResponse } from "@/lib/types";
import { NextApiRequest, NextApiResponse } from "next";
import { NextRequest, NextResponse } from "next/server";

type RequestPayload = {
    code?: string;
    currentChoice?: CurrentChoice;
};

export async function POST(req: NextRequest) {
    const { code, currentChoice }: RequestPayload = await req.json() as RequestPayload;
    if (!code || !currentChoice) {
        console.log(code, currentChoice);
        return NextResponse.json({}, {status: 400});
    }

    const url = process.env.CODERUNNER_BACKEND_URL as string;
    const api = "/api/v1/evaluate";
    if (!url) {
        console.log('Coderunner backend API URL is not set');
        return NextResponse.json({}, {status: 500});
    }
    try {
        const response = await fetch(`${url}${api}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify({ code, ...currentChoice })
        });
        return NextResponse.json(await response.json());
    } catch (e) {
        console.log('Failed to compile code');
        console.log(' - Backend URL', url);
        console.log(' - Backend API', api);
        console.log(e);
        return NextResponse.json({}, {status: 500});
    }
}
