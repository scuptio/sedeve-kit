Install venv
```
apt install python3.10-venv
```

To build this package, first install requirements:

```
pip install -r requirements.txt
```

Create a new directory containing a new Python `virtualenv`
```
python3 -m venv .env
source .env/bin/activate
```

To build and test use maturin develop:
```
maturin develop
pytest
```


