<?php

namespace App\View\Components\pages\inventory;

use Closure;
use Illuminate\Contracts\View\View;
use Illuminate\View\Component;

class items_list extends Component
{
    /**
     * Create a new component instance.
     */
    public function __construct()
    {
        //
    }

    /**
     * Get the view / contents that represent the component.
     */
    public function render(): View|Closure|string
    {
        return view('components.pages.inventory.items_list');
    }
}
