export interface Message {
  id: string;
  from: string;
  text: string;
  timestamp: number;
}

export interface IngestRequest {
  from: string;
  text: string;
  id?: string;
  timestamp?: number;
}

export interface IngestResponse {
  ok: boolean;
  stored: Message;
  reply: Message;
}

export interface RelayResponse {
  local: Message;
  peer: IngestResponse;
}

export interface DemoStep {
  step: number;
  actor: string;
  action: string;
  payload: unknown;
}

export interface DemoResponse {
  steps: DemoStep[];
}
