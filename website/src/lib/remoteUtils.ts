import type { LangKey, LinkData, Result } from "$lib/types";

const TOO_MANY_REQUESTS = 429;

export const sendCodeToServer = async (
  code: string,
  language: LangKey,
  compiler?: string,
  executor?: string,
  compilerOptions?: string,
): Promise<Result> => {
  const body = JSON.stringify({
    code,
    language,
    compiler,
    compiler_args: compilerOptions?.split(" "),
    executor,
  });
  const response = await fetch("/api/code-eval", {
    method: "POST",
    body,
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });
  if (response.ok) {
    return await response.json();
  } else if (response.status === TOO_MANY_REQUESTS) {
    throw new Error("Too many requests, wait a moment and try again.");
  } else {
    console.error("Error response from server:", response.status);
    throw new Error(
      "Server error, could not evaluate code (we are sorry and are working on it!)",
    );
  }
};

export const generateNewLink = async (data: LinkData): Promise<string> => {
  const body = JSON.stringify(data);

  const response = await fetch("/api/link", {
    method: "POST",
    body,
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
  });

  if (response.ok) {
    const { key } = await response.json();
    return key;
  } else if (response.status === TOO_MANY_REQUESTS) {
    throw new Error("Too many requests, please try again later.");
  } else {
    throw new Error("Could not generate link");
  }
};

export const getLinkData = async (key: string): Promise<LinkData> => {
  const response = await fetch(`/api/link/${key}`, {
    method: "GET",
    mode: "cors",
    headers: {
      Accept: "application/json",
    },
  });

  if (response.ok) {
    const { value } = await response.json();
    return JSON.parse(value);
  } else {
    throw new Error("Could not fetch link data");
  }
};
