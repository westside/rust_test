using Bhaptics.Test;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TestScript : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        var vec = new Vec2();
        vec.x = 1f;
        vec.y = 2f;
        var res = Interop.my_function(vec);
        Debug.LogFormat("{0} {1}", res.x, res.y);
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
