from ast import Continue
from telnetlib import GA
import requests
import time, datetime
from dataclasses import dataclass, field
from typing import List, Tuple, Dict, Any, Set
from enum import Enum

BIND_ADDRESS: str = "http://localhost:8000/fetch-chess-data" 
N_USERS_TO_PROCESS_FOR_GAME_MODE: int = 50

class GameMode(Enum):
    BLITZ = 'blitz'
    RAPID = 'rapid'
    BULLET = 'bullet'
    
class UserRatingPair:
    def __init__(self, user_name: str, elo_rating: int):
        self.user_name: str = user_name
        self.elo_rating: int = elo_rating
    
    def __iter__(self):
        yield self.user_name
        yield self.elo_rating
        
@dataclass
class BackendRequestPayload:
    user_name: str
    games_count: int
    game_mode: GameMode
    user_color: str
    
    def to_json(self) -> Dict[str, Any]:
        return {
            "username": self.user_name,
            "games_count": self.games_count,
            "game_mode": self.game_mode.value,
            "user_color": self.user_color,
        }
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendRequestPayload":
        return BackendRequestPayload(
            user_name = data["username"],
            games_count = data["games_count"],
            game_mode = data["game_mode"],
            user_color = data["user_color"]
        )

@dataclass
class BackendResponse:
    average_time: float
    opponents_and_their_rating: List[UserRatingPair]
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendResponse":
        return BackendResponse(
            average_time = data["time"],
            opponents_and_their_rating = [
                UserRatingPair(opponent[0], opponent[1]) for opponent in data["players_considered"]
            ]
        )
        
class Processor:
    def __init__(self):
        self.results: List[List[BackendResponse]] = []
    
    @staticmethod
    def get_starting_user_rating_pairs() -> List[UserRatingPair]:
        return [
            UserRatingPair("Hexaquarks1", 1900),
            UserRatingPair("fifthart", 1800) 
        ]
    
    class UserProcessor:
        def __init__(self, game_mode: GameMode):
            self.game_mode: GameMode = game_mode
            
            self.local_results: List[BackendResponse] = []
            
            # Set cache for usernames, to not process same user again
            self.processed_users: Set[str] = set() 
            
            # Variables for statistical purposes
            self.nb_processed_users_for_curr_game_mode: int = 0
            self.nb_total_considered_users_for_curr_game_mode: int = 0
            
        def process_user(
            self,
            user_name: str,
            games_count: int,
            game_mode: GameMode, 
            user_color: str) -> BackendResponse:
            
            payload = BackendRequestPayload(
                user_name = user_name,
                games_count = games_count,
                game_mode = game_mode,
                user_color = user_color
            )
            try:
                response = requests.post(
                    BIND_ADDRESS, 
                    headers = {
                        "Content-Type": 'application/json',
                        "x-requested-by": "internal"
                    },
                    json = payload.to_json())
                response.raise_for_status() 
                response_data = response.json()
                
                return BackendResponse.from_json(response_data)
            except Exception as e:
                print(f"A server error occured while trying toprocess user {user_name}: {e}")
                raise
            
        def process_users_for_game_mode(self):
            # Reset timer
            start_time = time.time()
            
            # Reinitialize the root usernames
            user_rating_pairs: List[UserRatingPair] = Processor.get_starting_user_rating_pairs()
            
            while len(user_rating_pairs) != 0:
                if self.nb_processed_users_for_curr_game_mode > N_USERS_TO_PROCESS_FOR_GAME_MODE:
                    break
                
                self.nb_total_considered_users_for_curr_game_mode += 1
                
                user_info = user_rating_pairs.pop()
                user_name = user_info.user_name
                
                if user_name in self.processed_users:
                    continue 
                    
                print(f"[{datetime.datetime.now()}] Game mode {self.game_mode} - processing user: {user_name}")
            
                try: 
                    result: BackendResponse = self.process_user(user_name, 10, self.game_mode, "both")
                    self.local_results.append(result)
                    self.nb_processed_users_for_curr_game_mode += 1
                    
                    user_rating_pairs.extend(result.opponents_and_their_rating)
                    print(f"[{datetime.datetime.now()}] Game mode {self.game_mode} - Successfully processed user: {user_name}")
                    
                except Exception as e:
                    # We don't really care what kind of error was returned from the server, I think.
                    # Indeed, this procedure is for internal use only. Here I can afford to just
                    # reject the erroneous user and add it to the processed users set
                    
                    print(f"[{datetime.datetime.now()}] Game mode {self.game_mode} - An error occurred while trying to process player {user_name}. It was skipped in the analysis")
                
                self.processed_users.add(user_name)
                
            end_time = time.time()
            total_time = end_time - start_time
            
            print(f"Total execution time for game mode: {self.game_mode} is: {total_time:.2f} seconds")
            print(f"===== Finished running game mode: {self.game_mode} =====")

    def process_users_for_all_game_modes(self):
        print(f"=====  Starting script  =====")
        for game_mode in GameMode:
            userProcessor = self.UserProcessor(game_mode)
            userProcessor.process_users_for_game_mode()
            
            self.results.append(userProcessor.local_results)
            
        print(f"=====  Script finished running. Results:  =====")

## Need to keep info on processed elo brakets

if __name__ == "__main__":
    processor = Processor()
    processor.process_users_for_all_game_modes()
