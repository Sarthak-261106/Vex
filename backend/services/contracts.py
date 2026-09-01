"""Replaceable service contracts. Implementations must uphold AGENTS.md rules."""
from abc import ABC, abstractmethod

class AIService(ABC):
    @abstractmethod
    def respond(self, prompt: str) -> str: ...

class LocalLLMService(AIService): pass
class VisionService(ABC):
    @abstractmethod
    def observe(self): ...
class SpeechRecognitionService(ABC): pass
class TextToSpeechService(ABC): pass

class ServiceRegistry:
    @staticmethod
    def phase_one_status() -> str:
        return "No AI, vision, voice, memory, or automation service is active."
