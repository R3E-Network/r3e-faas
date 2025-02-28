export declare type EventHandler = (eventName: string, data: any) => {};
export interface Message {
    platform: string;
    blockchain: string;
    version: string;
    messageId: string;
    command: string;
    data?: any;
    network: string;
}
export interface IncomingMessage extends Message {
    eventName?: string;
    error?: string;
}
export interface AddEventsListenerArgs {
    blockchain: string;
    callback: EventHandler;
}
export interface SendMessageArgs {
    blockchain: string;
    version: string;
    command: string;
    data?: any;
    network: string;
    timeout?: number;
}
export declare type SendMessage = (args: SendMessageArgs) => Promise<any>;
export declare type AddEventsListener = (args: AddEventsListenerArgs) => void;
export interface Plugin {
    blockchain: string;
    new (sendMessage: SendMessage, addEventsListener: AddEventsListener): Plugin;
}
//# sourceMappingURL=types.d.ts.map