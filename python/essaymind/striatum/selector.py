from body.node import BodyNode
#from body.area import Area

import logging
from body.fiber import FiberKey, FiberKeyValue

log = logging.getLogger('Select')

class Selector(BodyNode):
    def __init__(self, area, name):
        super().__init__(area, name)
        self.area = area
        self.name = name
        
        #area[name] = self
        
        self.output_map = dict()
        self.select_items = []
        self.unselect_items = []
        
        self.select_tick_max = 2
        self.unselect_tick_max = 2
        
        self.interrupt_tick_max = 4
        self.interrupt_tick = 0
        
        self.active_select_items = []
        self.active_unselect_items = []
        
        self._on_active = None
        self.is_activator = False
        self.is_active = True
        
    # build methods
    
    def choose(self, action, name=None):
        if not name:
            name = action.name
            
        output = SelectorOutput(self, name, action)
        self.output_map[name] = output
        
        return output
    
    def choose_value(self, action, name=None):
        if not name:
            name = action.name
            
        output = SelectorOutputValue(self, name, action)
        self.output_map[name] = output
        
        return output
    
    def choose_fiber_value(self, name):
        output = OutputFiberValue(self, name)
        self.output_map[name] = output
        
        return output
    
    def when(self, fiber):
        self.is_activator = True
        self.is_active = False
        fiber.to(self.activate)
        
        return self
        
    def add_notice(self, notice):
        #self.solution_notices.append(notice)
        self.on_active().to(notice)
        
    # active methods
    
    def add_select_item(self, item):
        self.select_items.append(item)
    
    def add_unselect_item(self, item):
        self.unselect_items.append(item)
    
    def add_active_select_item(self, item):
        assert False
        self.active_select_items.append(item)
    
    def add_active_unselect_item(self, item):
        self.active_unselect_items.append(item)
        
    def on_active(self):
        assert not self.area.is_build()
        
        if not self._on_active:
            self._on_active = self.nexus('on_active')
            
        return self._on_active
        
    def interrupt(self):
        self.interrupt_tick = self.interrupt_tick_max
        
    def activate(self):
        self.is_active = True
        
    def tick(self):
        is_active = self.is_active
        self.is_active = not self.is_activator
        
        active_select_items = self.active_select_items
        self.active_select_items = []
        
        active_unselect_items = self.active_unselect_items
        self.active_unselect_items = []
        
        if self.interrupt_tick > 0:
            self.interrupt_tick = max(0, self.interrupt_tick - 1)
            return

        # activated by monoamine (DA/5HT)        
        if not is_active:
            return
        
        unselect_output_items = []
        
        for item in active_unselect_items:
            unselect_output_items.append(item.output)
            
        best_output = None
        best_cost = -1
        
        for item in active_select_items:
            if item.output in unselect_output_items:
                continue
            
            if best_cost < item.p:
                best_cost = item.p
                best_output = item.output
            
        if best_output:
            best_output.select()
            
            if self._on_active:
                self._on_active()

class SelectorOutput(BodyNode):
    def __init__(self, selector, name, action):
        self.selector = selector
        self.name = name
        self.action = action
        
        selector[name] = self
        
        self.select_ticks = 0
        self.unselect_ticks = 0
        
    # build methods
        
    def when(self, fiber, p=1):
        item = SelectItem(self, p)
        fiber.to(item.select)
        self.selector.add_select_item(item)
        
        return self
        
    def unless(self, fiber, p=1):
        item = UnselectItem(self, p)
        fiber.to(item.unselect)
        self.selector.add_unselect_item(item)
        
        return self
    
    # active methods
    
    def select(self):
        self.select_ticks = self.selector.select_tick_max
    
    def unselect(self):
        self.unselect_ticks = self.selector.unselect_tick_max
        
    def tick(self):
        if self.select_ticks > 0 and self.unselect_ticks <= 0:
            self.action()
            
        self.select_ticks = max(0, self.select_ticks - 1)
        self.unselect_ticks = max(0, self.unselect_ticks - 1)

class SelectorOutputValue(BodyNode):
    def __init__(self, selector, name, action):
        self.selector = selector
        self.name = name
        self.action = action
        
        selector[name] = self
        
        self.select_ticks = 0
        self.unselect_ticks = 0
        
    # build methods
        
    def when(self, fiber, p=1):
        assert isinstance(fiber, FiberKeyValue)
        
        item = SelectItemValue(self, p)
        fiber.to(item.select)
        self.selector.add_select_item(item)
        
        return self
        
    def unless(self, fiber, p=1):
        item = UnselectItem(self, p)
        fiber.to(item.unselect)
        self.selector.add_unselect_item(item)
        
        return self
    
    # active methods
    
    def select(self, key, value, p):
        self.select_ticks = self.selector.select_tick_max
        self.value = value
    
    def unselect(self):
        self.unselect_ticks = self.selector.unselect_tick_max
        
    def tick(self):
        if self.select_ticks > 0 and self.unselect_ticks <= 0:
            p = 1
            self.action(self.value, p)
            
        self.select_ticks = max(0, self.select_ticks - 1)
        self.unselect_ticks = max(0, self.unselect_ticks - 1)

class OutputFiberValue(BodyNode):
    def __init__(self, selector, name):
        super().__init__(selector, name)
        self.selector = selector

        self._on_select = FiberKeyValue()
        
        self.select_ticks = 0
        self.unselect_ticks = 0
        
        self._keys = []
        self._context_keys = []
        self._enable_key = None
        
        self.contexts = []
        self._default_context = Context([])
        self._context = self._default_context
        #self.items = []
        self.value = None
        
    # build methods
        
    def context(self, fiber):
        if isinstance(fiber, FiberKey):
            in_context = InputContextKey(self)
            fiber.to(in_context)
        elif isinstance(fiber, FiberKeyValue):
            in_context = InputContextKeyValue(self)
            fiber.to(in_context)
        else:
            assert isinstance(fiber, FiberKey)
        
        return self
        
    def enable(self, fiber):
        assert isinstance(fiber, FiberKey)
        
        #in_enable = InputEnableKey(self)
        #fiber.to(in_enable)
        
        fiber.to(self.enable_key)
        
        return self
        
    def when(self, fiber, p=1):
        if isinstance(fiber, FiberKeyValue):
            item = ItemKeyValueInput(self)
            fiber.to(item)
            #self.selector.add_select_item(item)
        elif isinstance(fiber, FiberKey):
            item = ItemKeyInput(self)
            fiber.to(item)
            #self.selector.add_select_item(item)
        else:
            assert isinstance(fiber, FiberKeyValue) or isinstance(fiber, FiberKey)
        
        return self
        
    def unless(self, fiber, p=1):
        item = UnselectItem(self, p)
        fiber.to(item.unselect)
        self.selector.add_unselect_item(item)
        
        return self
    
    def on_select(self):
        return self._on_select
    
    def to(self, target):
        self.on_select().to(target)
    
    # active methods
    
    def send_key_value(self, key, value, p):
        log.info(f"send key value {key} {value}")
        self._keys.append((key, value, p))
    
    def send_key(self, key, p):
        self._keys.append((key, 0, p))
    
    def context_key(self, key, p):
        self._context_keys.append(key)
    
    def enable_key(self, key, p):
        self._enable_key = key
        
    def __call__(self, key, p):
        self.send_key(key, p)
        
    def learn(self):
        log.info(f"try-learn {self.last_keys}")
        
        if self.last_keys == None:
            return
        
        item = self._context.find(self.last_keys)
        
        if not item:
            item = Item(self.last_keys)
            self._context.add(item)
            log.info(f"learn new {item}")
            
    def learn_pair(self, context_key, key):
        log.info(f"learn-pair {context_key} {key}")
        if context_key:
            context = self.create_context([context_key])
        else:
            context = self._default_context
        context.create([key])
    
    def select(self, key, value, p):
        self.select_ticks = self.selector.select_tick_max
        self.value = value
    
    def unselect(self):
        self.unselect_ticks = self.selector.unselect_tick_max
        
    def tick(self):
        value = self.value
        self.value = None
        
        self.update_select(value)
        
        if self.select_ticks > 0 and self.unselect_ticks <= 0 and self.value != None:
            p = 1
            self._on_select(self.name, self.value, p)
            
        #self.select_ticks = max(0, self.select_ticks - 1)
        #self.unselect_ticks = max(0, self.unselect_ticks - 1)
                
    def update_select(self, value):
        enable_key = self._enable_key
        self._enable_key = None
        
        log.info(f"selector {self._context_keys} {self._keys}")
        
        if not len(self._context_keys):
            self._context = self._default_context
        else:
            self._context = self.create_context(self._context_keys)
                
            self.context_keys = []
            
        if not len(self._keys):
            self.last_keys = None
            return
        
        self._last_keys = keys = self._keys
        self._keys = []
        
        #if value == None:
        #    self.last_keys = None
        #    return
        
        self.process_keys(keys)
        
    def process_keys(self, keys):
        for in_key,value,p in keys:
            
            item = self._context.find_key(in_key)
            log.info(f"PK: {in_key},{value},{p} {self._context} {item}")
            if item:
                p = 1
                self.select(item.key, value, p)
        
    def process_keys_old(self, keys, value, enable_key):
        item = self._context.find(keys)

        if item:
            p = 1
            self.select(item.key, value, p)
        elif enable_key:
            p = 1
            # use value_key instead of enable_key?
            self.select(enable_key, value, p)
            
    def create_context(self, keys):
        context = self.find_context(keys)
            
        if not context:
            context = Context(keys)
            self.contexts.append(context)
            
        return context
        
            
    def find_context(self, keys):
        for context in self.contexts:
            if context.is_match(keys):
                return context
            
        return None

class Context:
    def __init__(self, keys, name='context'):
        self.keys = keys
        
        self.items = []
        
    def add(self, item):
        self.items.append(item)
        
    def is_match(self, keys):
        for key in self.keys:
            if not key in keys:
                return False
            
        return True
        
    def find(self, keys):
        for item in self.items:
            if item.is_match(keys):
                return item
            
        return None
        
    def find_key(self, key):
        for item in self.items:
            if item.is_match_key(key):
                return item
            
        return None
            
    def create(self, keys):
        item = self.find(keys)
            
        if not item:
            item = Item(keys)
            self.items.append(item)
            
        return item
    
    def __str__(self):
        return "Context[%s]" % ";".join(self.keys)
                
class Item:
    def __init__(self, keys):
        self.keys = keys
        self.key = ";".join(keys)
        
    def is_match_key(self, key):
        return key in self.keys
        
    def is_match_any(self, keys):
        for key in self.keys:
            if key in keys:
                return True
            
        return False
        
    def is_match_all(self, keys):
        for key in self.keys:
            if not key in keys:
                return False
            
        return True
    
    def __str__(self):
        return f"Item[{self.key}]"
        
class ItemKeyValueInput:
    def __init__(self, output):
        self.output = output
        
    def __call__(self, key, value, p):
        self.output.send_key_value(key, value, p)
        
class ItemKeyInput:
    def __init__(self, output):
        self.output = output
        
    def __call__(self, key, value, p):
        self.output.send_key(key, p)
        
class InputContextKey:
    def __init__(self, output):
        self.output = output
        
    def __call__(self, key, p):
        self.output.context_key(key, p)
        
class InputContextKeyValue:
    def __init__(self, output):
        self.output = output
        
    def __call__(self, key, value, p):
        self.output.context_key(key, p)
        
class InputEnableKey:
    def __init__(self, output):
        self.output = output
        
    def __call__(self, key, p):
        self.output.enable_key(key, p)
        
class SelectItem:
    def __init__(self, output, p):
        self.output = output
        self.selector = output.selector
        self.p = p
        
    def select(self):
        self.selector.add_active_select_item(self)
        
    def tick(self):
        self.output.select()
        
class SelectItemValue:
    def __init__(self, output, p):
        self.output = output
        self.selector = output.selector
        self.p = p
        
    def select(self, key, value, p):
        self.selector.add_active_select_item(self)
        #self.output.add_key_value(key, value)
        self.key = key
        self.value = value
        
    def tick(self):
        self.output.select(self.value)
        
class SelectItemKey:
    def __init__(self, output, p):
        self.output = output
        self.selector = output.selector
        self.p = p
        
    def select(self, key, value, p):
        self.selector.add_active_select_item(self)
        #self.output.add_key(key)
        self.key = key
        
    def tick(self):
        self.output.select(self.key)
        
class UnselectItem:
    def __init__(self, output, p):
        self.output = output
        self.selector = output.selector
        self.p = p
        
    def unselect(self):
        self.selector.add_active_unselect_item(self)
        
    def tick(self):
        self.output.unselect()
        