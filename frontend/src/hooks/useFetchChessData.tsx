import { useState } from 'react';
import { sendDataToBackend, ResponseInformation } from '../services/apiService';
import { InputProps } from '../components/panels/InputsPanel';

const useFetchChessData = () => {
    const [response, setResponse] = useState<ResponseInformation | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const fetchData = async (props: InputProps) => {
        setIsLoading(true);
        setError(null);

        try {
            const responseData: ResponseInformation = await sendDataToBackend(
                props.username,
                props.gamesCount,
                props.gameMode,
                props.userColor
            );
            setResponse(responseData);
        } catch (error) {
            setError(error instanceof Error ? error.message : 'An unexpected error occurred');
        } finally {
            setIsLoading(false);
        }
    };

    return { response, isLoading, error, fetchData };
};

export default useFetchChessData;