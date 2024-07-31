import { useState, useEffect } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';

interface WebSocketHookProps {
    setFetchProgress: (fetchProgress: string) => void;
    shouldConnectToWebsocket: boolean;
}
const useWebSocketHook = ({setFetchProgress, shouldConnectToWebsocket }: WebSocketHookProps) => {
    const [isWebsocketConnected, setIsWebsocketConnected] = useState(false);

    const END_POINT: string = 'ws://localhost:8000/ws';

    const { sendMessage, lastMessage, readyState, getWebSocket } = useWebSocket(END_POINT, {
        shouldReconnect: () => false, // No need, the life time of the WebSocket connection will be handled manually.
        onOpen: () => setIsWebsocketConnected(true),
        onClose: () => setIsWebsocketConnected(false),
        onMessage: (message) => setFetchProgress(message.data),
        share: true, // TODO: getWebSocket().close() asserts because of this, can't find a fix rn.
    }, shouldConnectToWebsocket);

    useEffect(() => {
        setIsWebsocketConnected(readyState === ReadyState.OPEN);
    }, [readyState as ReadyState]);

    useEffect(() => {
        if (lastMessage !== null) {
            // console.log('Received WebSocket message:', lastMessage.data);
            setFetchProgress(lastMessage.data);
        }
    }, [lastMessage]);

    const closeWebsocketConnection = () => {
        getWebSocket()?.close();
    };

    return { isWebsocketConnected, closeWebsocketConnection };
};

export default useWebSocketHook;
