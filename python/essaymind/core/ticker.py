import logging

log = logging.getLogger("Tick")

class TickManager(object):
    def __init__(self):
        self.tickers = []
        
        self.ticks = 0
        
    def add_ticker(self, ticker):
        assert isinstance(ticker, Ticker)
        
        self.tickers.append(ticker)
        
    def tick(self):
        ticks = self.ticks
        
        for ticker in self.tickers:
            ticker.tick(ticks)
            
        self.ticks = ticks + 1

class Ticker:
    def tick(self, ticks):
        return